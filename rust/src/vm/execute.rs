/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

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
use crate::vm::{inject_args::inject_args, prepare_vm::prepare_vm};
use smol_str::SmolStr;
#[inline(never)]
pub fn execute(
  mut bytecode: Vec<Instructions>,
  options: Option<RunOptions>,
) -> Result<Value, SmolStr> {
  let mut last_return = Value::Undefined;
  let mut stack: Vec<Value> = Vec::with_capacity(16);
  let var_count = resolve_symbols(&mut bytecode);
  let mut vars: Vec<Value> = vec![Value::Undefined; var_count];
  let mut _call_stack: Vec<usize> = Vec::new();
  let (functions, _exported, mut ip) = prepare_vm(&bytecode, &options);
  inject_args(&mut vars, &functions, &options, ip);
  let bytecode_ptr = bytecode.as_ptr();
  let bytecode_len = bytecode.len();
  while ip < bytecode_len {
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
      | Instructions::Truncate
      | Instructions::Import(_, _) => {
        stack_dispatch(instr, &mut stack, &mut vars, &options, ip).map_err(|e| e.format())?;
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
        math_dispatch(instr, &mut stack, &mut vars, ip).map_err(|e| e.format())?;
      }
      Instructions::Gt(_)
      | Instructions::Lt(_)
      | Instructions::Ge(_)
      | Instructions::Le(_)
      | Instructions::Eq(_)
      | Instructions::Neq(_) => {
        comparison_dispatch(instr, &mut stack, ip).map_err(|e| e.format())?;
      }
      Instructions::And | Instructions::Or | Instructions::Xor | Instructions::Not => {
        logic_dispatch(instr, &mut stack, ip).map_err(|e| e.format())?;
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
          &mut ip,
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
        io_dispatch(instr, &mut stack, ip).map_err(|e| e.format())?;
      }
      Instructions::MakeObj(_)
      | Instructions::MakeArray(_)
      | Instructions::AccessIndex
      | Instructions::Access(_)
      | Instructions::SetProp(_)
      | Instructions::Shrink => {
        collections_dispatch(instr, &mut stack, ip).map_err(|e| e.format())?;
      }
      Instructions::ToString
      | Instructions::ToShort
      | Instructions::ToInteger
      | Instructions::ToLong
      | Instructions::ToOcta
      | Instructions::ToHalf
      | Instructions::ToFloat
      | Instructions::ToDouble => {
        conversions_dispatch(instr, &mut stack, ip).map_err(|e| e.format())?;
      }
      Instructions::TypeOf | Instructions::Length => {
        metadata_dispatch(instr, &mut stack, ip).map_err(|e| e.format())?
      }
      Instructions::Nop
      | Instructions::Val(_)
      | Instructions::Set(_)
      | Instructions::Get(_)
      | Instructions::Inc(_, _)
      | Instructions::Dec(_, _) => {}
      _ => unsafe { std::hint::unreachable_unchecked() },
    }
    ip += 1;
  }
  if options.as_ref().is_some_and(|o| o.capture_return) {
    return Ok(last_return);
  }
  Ok(Value::Undefined)
}
