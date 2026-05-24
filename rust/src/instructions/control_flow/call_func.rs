/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::{FuncMetadata, Value};
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use smol_str::SmolStr;
#[inline]
pub fn call_func(
  name: &SmolStr,
  argc: u32,
  ip: &mut usize,
  stack: &mut Vec<Value>,
  call_stack: &mut Vec<usize>,
  vars: &mut Vec<Value>,
  functions: &AHashMap<SmolStr, FuncMetadata>,
) -> Result<(), VMError> {
  let fn_meta = functions.get(name).ok_or_else(|| VMError::InvalidOpcode {
    ip: *ip,
    code: SmolStr::from(format!("CALL {}", name)),
  })?;
  let argc_usize = argc as usize;
  if stack.len() < argc_usize {
    return Err(VMError::StackUnderflow {
      ip: *ip,
      opcode: "CALL_ARGS",
    });
  }
  call_stack.push(*ip);
  let start_idx = stack.len() - argc_usize;
  let params_count = fn_meta.params_count as usize;
  if vars.len() < params_count {
    vars.resize(params_count, Value::Undefined);
  }
  let mut arg_idx = 0;
  for val in stack.drain(start_idx..) {
    if arg_idx < params_count {
      vars[arg_idx] = val;
      arg_idx += 1;
    }
  }
  *ip = fn_meta.start;
  Ok(())
}
