/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn not_values(a: Value) -> Value {
  Value::Bool(!a.is_truthy())
}
#[inline]
pub fn not_func(stack: &mut SmallVec<[Value; 16]>, ip: usize) -> Result<(), VMError> {
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "AND" })?;
  let a = std::mem::take(a_ref);
  *a_ref = not_values(a);
  Ok(())
}
