/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::stack::import_func::import_func;
use crate::modules::krates::{
  gas_monitor::GasMonitor, validate_bytecode::validate_bytecode,
  validate_security::validate_security, validate_vars::validate_vars,
};
use crate::modules::torja::resolve_symbols::resolve_symbols;
use crate::types::{
  control_flow_signal::ControlFlowSignal,
  instructions::Instructions,
  value::{RunOptions, Value},
};
use crate::vm::dispatch::{
  collections_dispatch::collections_dispatch, comparison_dispatch::comparison_dispatch,
  control_flow_dispatch::control_flow_dispatch, conversions_dispatch::conversions_dispatch,
  io_dispatch::io_dispatch, logic_dispatch::logic_dispatch, math_dispatch::math_dispatch,
  metadata_dispatch::metadata_dispatch, stack_dispatch::stack_dispatch,
};
use crate::vm::{inject_args::inject_args, prepare_vm::prepare_vm};
use ahash::AHashMap;
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
#[cold]
#[inline(never)]
fn handle_unused_opcodes() {}
#[cold]
pub fn execute(
  mut bytecode: Vec<Instructions>,
  options: Option<RunOptions>,
  halt_flag: Option<Arc<AtomicBool>>,
) -> Result<(Value, u64), SmolStr> {
  let mut last_return = Value::Undefined;
  let mut stack: SmallVec<[Value; 128]> = SmallVec::new();
  let empty_map: AHashMap<SmolStr, Value> = AHashMap::new();
  let imports = options.as_ref().map(|o| &o.imports).unwrap_or(&empty_map);
  let (var_count, symbol_table) = resolve_symbols(&mut bytecode, imports);
  let mut vars: Vec<Value> = vec![Value::Undefined; var_count];
  for (name, val) in imports {
    if let Some(&idx) = symbol_table.get(name) {
      vars[idx] = val.clone();
    }
  }
  let mut _call_stack: Vec<usize> = Vec::new();
  let (functions, _exported, mut ip) = prepare_vm(&bytecode, &options);
  let security_config = options
    .as_ref()
    .map(|o| o.security_config.clone())
    .unwrap_or_default();
  validate_vars(&bytecode, var_count)?;
  validate_bytecode(&bytecode, &functions)?;
  validate_security(&bytecode, &security_config)?;
  inject_args(&mut vars, &functions, &options, ip);
  let bytecode_ptr = bytecode.as_ptr();
  let bytecode_len = bytecode.len();
  let threshold = if bytecode_len < 100 { 1 } else { 50 };
  let gas_monitor = GasMonitor::new(&security_config)?;
  let mut tick: u64 = 0;
  let mut runtime_io_count = 0usize;
  let mut runtime_call_count = 0usize;
  let mut runtime_jump_count = 0usize;
  let mut runtime_alloc_count = 0usize;
  let mut runtime_import_count = 0usize;
  while ip < bytecode_len {
    gas_monitor.check_tick(tick)?;
    if tick.is_multiple_of(threshold)
      && let Some(ref flag) = halt_flag
      && flag.load(Ordering::Relaxed)
    {
      return Ok((Value::Undefined, tick));
    }
    tick += 1;
    unsafe { std::hint::assert_unchecked(ip < bytecode_len) }
    debug_assert!(
      ip < bytecode_len,
      "IP out of bounds! IP: {}, Len: {}",
      ip,
      bytecode_len
    );
    let instr = unsafe { &*bytecode_ptr.add(ip) };
    match instr {
      Instructions::InitStack(_)
      | Instructions::PushInt16(_)
      | Instructions::PushInt32(_)
      | Instructions::PushInt64(_)
      | Instructions::PushInt128(_)
      | Instructions::PushFloat16(_)
      | Instructions::PushFloat32(_)
      | Instructions::PushFloat64(_)
      | Instructions::PushString(_)
      | Instructions::PushArray(_)
      | Instructions::PushObject(_)
      | Instructions::PushUndefined
      | Instructions::PushNull
      | Instructions::PushNaN
      | Instructions::Push(_)
      | Instructions::ValIdx(_)
      | Instructions::SetIdx(_)
      | Instructions::GetIdx(_)
      | Instructions::Concat
      | Instructions::Dup
      | Instructions::Swap
      | Instructions::Truncate => {
        stack_dispatch(instr, &mut stack, &mut vars, ip)?;
      }
      Instructions::Import(module_name, alias_idx) => {
        if !security_config.unsafe_mode {
          runtime_import_count += 1;
          if runtime_import_count > security_config.max_import {
            return Err(SmolStr::from("Security Violation: Excessive imports"));
          }
        }
        import_func(&mut vars, &options, module_name, *alias_idx, ip)?;
      }
      Instructions::Add(_)
      | Instructions::Sub(_)
      | Instructions::Mul(_)
      | Instructions::Div(_)
      | Instructions::Mod(_)
      | Instructions::Shl(_)
      | Instructions::Shr(_)
      | Instructions::Ror(_)
      | Instructions::Rol(_)
      | Instructions::Pow(_)
      | Instructions::Powi(_)
      | Instructions::Powf(_)
      | Instructions::Sin(_)
      | Instructions::Cos(_)
      | Instructions::Tan(_)
      | Instructions::Neg(_)
      | Instructions::IncIdx(_, _)
      | Instructions::DecIdx(_, _) => {
        math_dispatch(instr, &mut stack, &mut vars, ip)?;
      }
      Instructions::Gt(_)
      | Instructions::Lt(_)
      | Instructions::Ge(_)
      | Instructions::Le(_)
      | Instructions::Eq(_)
      | Instructions::Neq(_) => {
        comparison_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::And | Instructions::Or | Instructions::Xor | Instructions::Not => {
        logic_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::IfFalse(_) | Instructions::Jump(_) | Instructions::Break(_) => {
        if !security_config.unsafe_mode {
          runtime_jump_count += 1;
          if runtime_jump_count > security_config.max_jump {
            return Err(SmolStr::from("Security Violation: Excessive jumps"));
          }
        }
        match control_flow_dispatch(
          instr,
          &mut stack,
          &mut vars,
          &mut _call_stack,
          &mut last_return,
          &functions,
          &symbol_table,
          &mut ip,
          bytecode_len,
        )? {
          ControlFlowSignal::Continue => continue,
          ControlFlowSignal::Break => break,
          ControlFlowSignal::None => {}
        }
      }
      Instructions::Call(_, _) => {
        if !security_config.unsafe_mode {
          runtime_call_count += 1;
          if runtime_call_count > security_config.max_call {
            return Err(SmolStr::from("Security Violation: Excessive calls"));
          }
        }
        match control_flow_dispatch(
          instr,
          &mut stack,
          &mut vars,
          &mut _call_stack,
          &mut last_return,
          &functions,
          &symbol_table,
          &mut ip,
          bytecode_len,
        )? {
          ControlFlowSignal::Continue => continue,
          ControlFlowSignal::Break => break,
          ControlFlowSignal::None => {}
        }
      }
      Instructions::Return
      | Instructions::Stop
      | Instructions::Instantiate(_, _)
      | Instructions::Func(_, _, _, _, _) => {
        match control_flow_dispatch(
          instr,
          &mut stack,
          &mut vars,
          &mut _call_stack,
          &mut last_return,
          &functions,
          &symbol_table,
          &mut ip,
          bytecode_len,
        )? {
          ControlFlowSignal::Continue => continue,
          ControlFlowSignal::Break => break,
          ControlFlowSignal::None => {}
        }
      }
      Instructions::Print
      | Instructions::Println
      | Instructions::Stdout
      | Instructions::Stdoutln
      | Instructions::Stdin
      | Instructions::InspectObj
      | Instructions::InspectArr
      | Instructions::ClearScreen => {
        if !security_config.unsafe_mode {
          runtime_io_count += 1;
          if runtime_io_count > security_config.max_io {
            return Err(SmolStr::from(format!(
              "Security Violation: I/O Flood at IP {}",
              ip
            )));
          }
        }
        io_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::MakeObj(_) | Instructions::MakeArray(_) => {
        if !security_config.unsafe_mode {
          runtime_alloc_count += 1;
          if runtime_alloc_count > security_config.max_alloc {
            return Err(SmolStr::from("Security Violation: Memory limit reached"));
          }
        }
        collections_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::AccessIndex
      | Instructions::Access(_)
      | Instructions::SetProp(_)
      | Instructions::Shrink => {
        collections_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::ToString
      | Instructions::ToShort
      | Instructions::ToInteger
      | Instructions::ToLong
      | Instructions::ToOcta
      | Instructions::ToHalf
      | Instructions::ToFloat
      | Instructions::ToDouble => {
        conversions_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::TypeOf | Instructions::Length => metadata_dispatch(instr, &mut stack, ip)?,
      Instructions::Nop
      | Instructions::Export(_)
      | Instructions::Val(_)
      | Instructions::Set(_)
      | Instructions::Get(_)
      | Instructions::Inc(_, _)
      | Instructions::Dec(_, _) => {
        handle_unused_opcodes();
      }
      _ => unsafe { std::hint::unreachable_unchecked() },
    }
    ip += 1;
  }
  if options.as_ref().is_some_and(|o| o.capture_return) {
    if let Value::Undefined = last_return
      && let Some(v) = stack.pop()
    {
      last_return = v;
    }
    return Ok((last_return, tick));
  }
  Ok((Value::Undefined, tick))
}
#[test]
fn test_execute_basic_math_and_return() {
  let bytecode = vec![
    Instructions::PushInt32(5),
    Instructions::PushInt32(10),
    Instructions::Add(crate::types::primitive_types::PrimitiveTypes::Int),
    Instructions::Stop,
  ];
  let halt_flag = Arc::new(AtomicBool::new(false));
  let options = crate::types::value::RunOptions {
    capture_return: true,
    ..Default::default()
  };
  let result = execute(bytecode, Some(options), Some(halt_flag));
  assert!(result.is_ok());
  let (val, _tick) = result.unwrap();
  assert_eq!(val, Value::Int32(15));
}
#[test]
fn test_halt_flag_behavior() {
  let bytecode = vec![Instructions::Jump(0)];
  let halt_flag = Arc::new(AtomicBool::new(false));
  let flag_clone = halt_flag.clone();
  flag_clone.store(true, std::sync::atomic::Ordering::Relaxed);
  let result = execute(bytecode, None, Some(halt_flag));
  assert!(result.is_ok());
  let (val, _tick) = result.unwrap();
  assert_eq!(val, Value::Undefined);
}
