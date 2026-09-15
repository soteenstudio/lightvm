/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::root::cbrt::{
  cbrt_f16in::cbrt_f16in, cbrt_f32in::cbrt_f32in, cbrt_f64in::cbrt_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn cbrt_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  if !matches!(
    &a,
    Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(a.clone()),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Float16(cbrt_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(cbrt_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(cbrt_f64in(a.as_f64())),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: "unknown",
      });
    }
  })
}
#[inline]
pub fn cbrt_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "CBRT" })?;
  let result = cbrt_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn rejects_integer_operands_without_mutating_stack() {
    let value = Value::Int64(1);
    assert!(matches!(
      cbrt_values(value.clone(), PrimitiveTypes::Dbl, 21),
      Err(VMError::TypeMismatch {
        ip: 21,
        expected: "Double",
        found: "Long"
      })
    ));
    let mut stack = Stack::from_vec(vec![value]);
    let original = stack.clone();
    assert!(matches!(
      cbrt_func(&mut stack, PrimitiveTypes::Dbl, 22),
      Err(VMError::TypeMismatch {
        ip: 22,
        expected: "Double",
        found: "Long"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn reports_errors_without_mutating_stack() {
    crate::instructions::math::assert_unary_float_errors(super::cbrt_func, "CBRT");
  }
}
