/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::exp::{
  exp_f16in::exp_f16in, exp_f32in::exp_f32in, exp_f64in::exp_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::expected_type::expected_type;
#[inline(always)]
pub fn exp_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  if !matches!(
    &a,
    Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: a.type_of(),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Float16(exp_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(exp_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(exp_f64in(a.as_f64())),
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
pub fn exp_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "EXP" })?;
  let result = exp_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn reports_errors_without_mutating_stack() {
    crate::instructions::math::assert_unary_float_errors(super::exp_func, "EXP");
  }
  #[test]
  fn rejects_non_float_operands_without_mutating_stack() {
    for (value, found) in [
      (Value::Int16(1), "int16"),
      (Value::String("invalid".into()), "string"),
    ] {
      assert!(matches!(
        exp_values(value.clone(), PrimitiveTypes::Flt, 20),
        Err(VMError::TypeMismatch { ip: 20, expected: "Float", found: actual })
          if actual == found
      ));
      let mut stack = Stack::from_vec(vec![value]);
      let original = stack.clone();
      assert!(matches!(
        exp_func(&mut stack, PrimitiveTypes::Flt, 21),
        Err(VMError::TypeMismatch { ip: 21, expected: "Float", found: actual })
          if actual == found
      ));
      assert_eq!(stack, original);
    }
    let value = Value::Float32(1.0);
    assert!(matches!(
      exp_values(value.clone(), PrimitiveTypes::Int, 22),
      Err(VMError::TypeMismatch {
        ip: 22,
        found: "unknown",
        ..
      })
    ));
    let mut stack = Stack::from_vec(vec![value]);
    let original = stack.clone();
    assert!(matches!(
      exp_func(&mut stack, PrimitiveTypes::Int, 23),
      Err(VMError::TypeMismatch {
        ip: 23,
        found: "unknown",
        ..
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn accepts_cross_width_float_operands_and_returns_directive_type() {
    assert!(matches!(
      exp_values(Value::Float16(half::f16::ZERO), PrimitiveTypes::Flt, 22),
      Ok(Value::Float32(1.0))
    ));
    assert!(matches!(
      exp_values(Value::Float64(0.0), PrimitiveTypes::Hlf, 23),
      Ok(Value::Float16(value)) if value == half::f16::ONE
    ));
    assert!(matches!(
      exp_values(Value::Float32(0.0), PrimitiveTypes::Dbl, 24),
      Ok(Value::Float64(1.0))
    ));
  }
}
