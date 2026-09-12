/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::normalize::{
  normalize_f16in::normalize_f16in, normalize_f32in::normalize_f32in,
  normalize_f64in::normalize_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};

#[inline(always)]
pub fn normalize_values(
  value: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  let values = value.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::Float),
    found: get_type_name(value.clone()),
  })?;
  for value in values.iter() {
    if !value.is_number() {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: get_type_name(value.clone()),
      });
    }
  }

  Ok(match num_type {
    PrimitiveTypes::Hlf => normalize_f16in(values),
    PrimitiveTypes::Flt => normalize_f32in(values),
    PrimitiveTypes::Dbl => normalize_f64in(values),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: expected_type(num_type, ExpectedCategory::All),
      });
    }
  })
}

#[inline]
pub fn normalize_func(
  stack: &mut Stack,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let value = stack.last().cloned().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "NORMALIZE",
  })?;
  let result = normalize_values(value, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;

  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }

  #[test]
  fn normalizes_non_zero_vector() {
    let result = normalize_values(
      array(vec![Value::Float32(3.0), Value::Float32(4.0)]),
      PrimitiveTypes::Flt,
      0,
    )
    .unwrap();
    let values = result.as_array().unwrap();
    assert!((values[0].as_f32() - 0.6).abs() < f32::EPSILON);
    assert!((values[1].as_f32() - 0.8).abs() < f32::EPSILON);
  }

  #[test]
  fn normalizes_zero_vector_to_zero_vector() {
    let result = normalize_values(
      array(vec![Value::Float64(0.0), Value::Float64(0.0)]),
      PrimitiveTypes::Dbl,
      0,
    )
    .unwrap();
    assert_eq!(
      result,
      array(vec![Value::Float64(0.0), Value::Float64(0.0)])
    );
  }

  #[test]
  fn rejects_invalid_operands_without_mutating_stack() {
    for value in [Value::Bool(false), array(vec![Value::Bool(false)])] {
      let mut stack = Stack::from_vec(vec![value]);
      let original = stack.clone();
      assert!(matches!(
        normalize_func(&mut stack, PrimitiveTypes::Flt, 17),
        Err(VMError::TypeMismatch { ip: 17, .. })
      ));
      assert_eq!(stack, original);
    }
    assert!(matches!(
      normalize_values(array(vec![Value::Float32(1.0)]), PrimitiveTypes::Int, 18),
      Err(VMError::TypeMismatch { ip: 18, .. })
    ));
  }

  #[test]
  fn rejects_missing_operand() {
    let mut stack = Stack::new();
    assert!(matches!(
      normalize_func(&mut stack, PrimitiveTypes::Flt, 19),
      Err(VMError::StackUnderflow {
        ip: 19,
        opcode: "NORMALIZE"
      })
    ));
  }
}
