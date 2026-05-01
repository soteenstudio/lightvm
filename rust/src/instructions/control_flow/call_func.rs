/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::{FuncMetadata, Value};
#[inline]
pub fn call_func(
  name: &String,
  argc: u32,
  ip: &mut usize,
  stack: &mut Vec<Value>,
  call_stack: &mut Vec<usize>,
  vars: &mut Vec<Value>,
  functions: &std::collections::HashMap<String, FuncMetadata>,
) -> Result<(), String> {
  let fn_meta = functions
    .get(name)
    .ok_or_else(|| format!("Function {} not found", name))?;
  let mut args = Vec::new();
  for _ in 0..argc {
    args.push(stack.pop().ok_or("Stack underflow on CALL arguments")?);
  }
  args.reverse();
  call_stack.push(*ip);
  if stack.len() > 50 {
    stack.truncate(50);
  }
  for i in 0..fn_meta.params_count as usize {
    let val = args.get(i).cloned().unwrap_or(Value::Undefined);
    if i < vars.len() {
      vars[i] = val;
    }
  }
  *ip = fn_meta.start;
  Ok(())
}
