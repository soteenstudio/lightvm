/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::sin::{
  sin_f16in::sin_f16in, sin_f32in::sin_f32in, sin_f64in::sin_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn sin_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
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
    PrimitiveTypes::Hlf => Value::Float16(sin_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(sin_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(sin_f64in(a.as_f64())),
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
pub fn sin_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SIN" })?;
  let result = sin_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn invalid_operand_reports_type_mismatch_and_preserves_stack() {
    let invalid = Value::String("invalid".into());
    assert!(matches!(
      sin_values(invalid.clone(), PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Float",
        found: "String"
      })
    ));
    assert!(matches!(
      sin_values(Value::Int32(1), PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Float",
        found: "Integer"
      })
    ));
    let mut stack = Stack::from_vec(vec![invalid]);
    let original = stack.clone();
    assert!(matches!(
      sin_func(&mut stack, PrimitiveTypes::Flt, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Float",
        found: "String"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn accepts_cross_width_float_operands_and_returns_directive_type() {
    assert!(matches!(
      sin_values(Value::Float16(half::f16::ZERO), PrimitiveTypes::Flt, 19),
      Ok(Value::Float32(0.0))
    ));
    assert!(matches!(
      sin_values(Value::Float64(0.0), PrimitiveTypes::Hlf, 20),
      Ok(Value::Float16(value)) if value == half::f16::ZERO
    ));
    assert!(matches!(
      sin_values(Value::Float64(0.0), PrimitiveTypes::Flt, 21),
      Ok(Value::Float32(0.0))
    ));
  }
}
