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
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn or_values(a: Value, b: Value) -> Value {
  Value::Bool(a.is_truthy() || b.is_truthy())
}
#[inline]
pub fn or_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "OR" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "OR" })?;
  let a = std::mem::take(a_ref);
  *a_ref = or_values(a, b);
  Ok(())
}
