/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::stack::import_func::import_func;
use crate::types::{
  control_flow_signal::ControlFlowSignal,
  instructions::Instructions,
  value::{RunOptions, Value},
};
use crate::utils::resolve_symbols::resolve_symbols;
use crate::vm::dispatch::{
  collections_dispatch::collections_dispatch, comparison_dispatch::comparison_dispatch,
  control_flow_dispatch::control_flow_dispatch, conversions_dispatch::conversions_dispatch,
  io_dispatch::io_dispatch, logic_dispatch::logic_dispatch, math_dispatch::math_dispatch,
  metadata_dispatch::metadata_dispatch, stack_dispatch::stack_dispatch,
};
use crate::vm::{
  inject_args::inject_args, prepare_vm::prepare_vm, validate_bytecode::validate_bytecode,
  validate_vars::validate_vars,
};
use ahash::AHashMap;
use smallvec::SmallVec;
use smol_str::SmolStr;
#[cold]
#[inline(never)]
fn handle_unused_opcodes() {}
#[cold]
pub fn execute(
  mut bytecode: Vec<Instructions>,
  options: Option<RunOptions>,
) -> Result<Value, SmolStr> {
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
  validate_vars(&bytecode, var_count)?;
  validate_bytecode(&bytecode, &functions)?;
  inject_args(&mut vars, &functions, &options, ip);
  let bytecode_ptr = bytecode.as_ptr();
  let bytecode_len = bytecode.len();
  while ip < bytecode_len {
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
      Instructions::IfFalse(_)
      | Instructions::Jump(_)
      | Instructions::Return
      | Instructions::Call(_, _)
      | Instructions::Stop
      | Instructions::Instantiate(_, _)
      | Instructions::Break(_)
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
        io_dispatch(instr, &mut stack, ip)?;
      }
      Instructions::MakeObj(_)
      | Instructions::MakeArray(_)
      | Instructions::AccessIndex
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
    return Ok(last_return);
  }
  Ok(Value::Undefined)
}
