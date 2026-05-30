/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::stack::Stack;
use crate::types::value::{FuncMetadata, Value};
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use smol_str::SmolStr;
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn call_func(
  name: &SmolStr,
  argc: u32,
  ip: &mut usize,
  stack: &mut Stack,
  call_stack: &mut Vec<usize>,
  vars: &mut [Value],
  functions: &AHashMap<SmolStr, FuncMetadata>,
  symbol_table: &AHashMap<SmolStr, usize>,
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
  let mut args = Vec::with_capacity(argc_usize);
  for _ in 0..argc_usize {
    args.push(stack.pop().unwrap());
  }
  args.reverse();
  for (i, val) in args.into_iter().enumerate() {
    if i < fn_meta.param_names.len() {
      let param_name = &fn_meta.param_names[i];
      if let Some(&idx) = symbol_table.get(param_name)
        && idx < vars.len()
      {
        vars[idx] = val;
      }
    }
  }
  *ip = fn_meta.start;
  Ok(())
}
