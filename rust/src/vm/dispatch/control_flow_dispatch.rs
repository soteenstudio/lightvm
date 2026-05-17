/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::control_flow::{
  break_func::break_func, call_func::call_func, if_false_func::if_false_func,
  instantiate_func::instantiate_func, jump_func::jump_func, return_func::return_func,
  stop_func::stop_func,
};
use crate::types::{
  control_flow_signal::ControlFlowSignal,
  instructions::Instructions,
  value::{FuncMetadata, Value},
};
use ahash::AHashMap;
use smol_str::SmolStr;
pub fn control_flow_dispatch(
  instr: &Instructions,
  stack: &mut Vec<Value>,
  vars: &mut Vec<Value>,
  call_stack: &mut Vec<usize>,
  last_return: &mut Value,
  functions: &AHashMap<SmolStr, FuncMetadata>,
  ip: &mut usize,
) -> Result<ControlFlowSignal, SmolStr> {
  match instr {
    Instructions::IfFalse(target_ip) => {
      let cond = if_false_func(stack, *ip).map_err(|e| e.format())?;
      if cond {
        *ip = *target_ip;
        return Ok(ControlFlowSignal::Continue);
      }
      Ok(ControlFlowSignal::None)
    }
    Instructions::Jump(target_ip) => {
      jump_func(ip, *target_ip);
      Ok(ControlFlowSignal::Continue)
    }
    Instructions::Return => {
      if return_func(stack, call_stack, ip, last_return) {
        Ok(ControlFlowSignal::Continue)
      } else {
        Ok(ControlFlowSignal::Break)
      }
    }
    Instructions::Call(name, argc) => {
      call_func(name, *argc, ip, stack, call_stack, vars, &functions).map_err(|e| e.format())?;
      Ok(ControlFlowSignal::Continue)
    }
    Instructions::Stop => {
      if stop_func(call_stack, ip) {
        Ok(ControlFlowSignal::Continue)
      } else {
        Ok(ControlFlowSignal::Break)
      }
    }
    Instructions::Instantiate(class_name, argc) => {
      let instance =
        instantiate_func(stack, vars, class_name, *argc, *ip).map_err(|e| e.format())?;
      stack.push(instance);
      Ok(ControlFlowSignal::None)
    }
    Instructions::Break(target_ip) => {
      break_func(ip, *target_ip);
      Ok(ControlFlowSignal::Continue)
    }
    Instructions::Func(_name, _params, _start, end, _names) => {
      *ip = *end;
      Ok(ControlFlowSignal::Continue)
    }
    _ => Ok(ControlFlowSignal::None),
  }
}
