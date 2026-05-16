/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn and_values(a: Value, b: Value) -> Value {
  Value::Bool(a.is_truthy() && b.is_truthy())
}
#[inline]
pub fn and_func(stack: &mut Vec<Value>, ip: usize) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "AND" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "AND" })?;
  let a = std::mem::take(a_ref);
  *a_ref = and_values(a, b);
  Ok(())
}
