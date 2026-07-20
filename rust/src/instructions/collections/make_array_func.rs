/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use std::sync::Arc;
#[cold]
#[inline(never)]
pub fn make_array_func(stack: &mut Stack, count: u32, ip: usize) -> Result<(), VMError> {
  let count = count as usize;
  if stack.len() < count {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "MAKE_ARRAY",
    });
  }
  let start_index = stack.len() - count;
  let elements: Vec<Value> = stack.drain(start_index..).collect();
  stack.push(Value::Array(Arc::new(elements)));
  Ok(())
}
