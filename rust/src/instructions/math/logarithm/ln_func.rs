/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::logarithm::ln::{
  ln_f16in::ln_f16in, ln_f32in::ln_f32in, ln_f64in::ln_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn ln_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
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
    PrimitiveTypes::Hlf => Value::Float16(ln_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(ln_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(ln_f64in(a.as_f64())),
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
pub fn ln_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "LN" })?;
  let result = ln_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn rejects_non_float_operands_without_mutating_stack() {
    for (value, found) in [
      (Value::Int32(1), "Integer"),
      (Value::String("invalid".into()), "String"),
    ] {
      assert!(matches!(
        ln_values(value.clone(), PrimitiveTypes::Flt, 17),
        Err(VMError::TypeMismatch { ip: 17, expected: "Float", found: actual })
          if actual == found
      ));
      let mut stack = Stack::from_vec(vec![value]);
      let original = stack.clone();
      assert!(matches!(
        ln_func(&mut stack, PrimitiveTypes::Flt, 18),
        Err(VMError::TypeMismatch { ip: 18, expected: "Float", found: actual })
          if actual == found
      ));
      assert_eq!(stack, original);
    }
  }
  #[test]
  fn accepts_cross_width_float_operands_and_returns_directive_type() {
    assert!(matches!(
      ln_values(Value::Float16(half::f16::ONE), PrimitiveTypes::Dbl, 19),
      Ok(Value::Float64(0.0))
    ));
    assert!(matches!(
      ln_values(Value::Float64(1.0), PrimitiveTypes::Hlf, 20),
      Ok(Value::Float16(value)) if value == half::f16::ZERO
    ));
  }
  #[test]
  fn unsupported_directive_reports_unknown_without_mutating_stack() {
    assert!(matches!(
      ln_values(Value::Float32(1.0), PrimitiveTypes::Int, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        expected: "Float",
        found: "unknown"
      })
    ));
    let mut stack = Stack::from_vec(vec![Value::Float32(1.0)]);
    let original = stack.clone();
    assert!(ln_func(&mut stack, PrimitiveTypes::Int, 20).is_err());
    assert_eq!(stack, original);
  }
}
