/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use super::eq_func::eq_values;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn neq_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  let is_equal = eq_values(a, b, num_type);
  if let Value::Bool(val) = is_equal {
    Value::Bool(!val)
  } else {
    Value::Bool(true)
  }
}
#[inline]
pub fn neq_func(
  stack: &mut SmallVec<[Value; 16]>,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEQ" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEQ" })?;
  let a = std::mem::take(a_ref);
  *a_ref = neq_values(a, b, num_type);
  Ok(())
}
