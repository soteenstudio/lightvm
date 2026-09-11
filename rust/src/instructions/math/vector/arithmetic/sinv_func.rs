/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::sinv::{
  sinv_f16in::sinv_f16in, sinv_f32in::sinv_f32in, sinv_f64in::sinv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn sinv_values(a_val: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  let arr_a = a_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::Float),
    found: get_type_name(a_val.clone()),
  })?;
  for value in arr_a.iter() {
    if !value.is_number() {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Array(sinv_f16in(&arr_a)),
    PrimitiveTypes::Flt => Value::Array(sinv_f32in(&arr_a)),
    PrimitiveTypes::Dbl => Value::Array(sinv_f64in(&arr_a)),
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
pub fn sinv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let value = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SINV" })?;
  let result = sinv_values(value, num_type, ip)?;
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
  fn reports_type_mismatch_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![Value::Bool(false)]);
    let original = stack.clone();
    assert!(matches!(
      sinv_func(&mut stack, PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch { ip: 17, .. })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn validates_elements_and_directives() {
    assert!(sinv_values(array(vec![Value::Float32(1.0)]), PrimitiveTypes::Flt, 11).is_ok());
    assert!(matches!(
      sinv_values(array(vec![Value::Bool(false)]), PrimitiveTypes::Flt, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        found: "Boolean",
        ..
      })
    ));
    assert!(matches!(
      sinv_values(array(vec![Value::Float32(1.0)]), PrimitiveTypes::Str, 20),
      Err(VMError::TypeMismatch { ip: 20, .. })
    ));
  }
  #[test]
  fn preserves_mathematical_nan_results() {
    let result = sinv_values(
      array(vec![Value::Float32(f32::NAN)]),
      PrimitiveTypes::Flt,
      21,
    );
    assert!(matches!(
      result,
      Ok(Value::Array(values)) if values[0].as_f32().is_nan()
    ));
  }
  #[test]
  fn underflow_preserves_stack() {
    let mut stack = Stack::new();
    let original = stack.clone();
    assert!(matches!(
      sinv_func(&mut stack, PrimitiveTypes::Flt, 23),
      Err(VMError::StackUnderflow {
        ip: 23,
        opcode: "SINV"
      })
    ));
    assert_eq!(stack, original);
  }
}
