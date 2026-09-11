/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::root::cbrtv::{
  cbrtv_f16in::cbrtv_f16in, cbrtv_f32in::cbrtv_f32in, cbrtv_f64in::cbrtv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};

#[inline(always)]
pub fn cbrtv_values(value: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
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
    PrimitiveTypes::Hlf => Value::Array(cbrtv_f16in(&values)),
    PrimitiveTypes::Flt => Value::Array(cbrtv_f32in(&values)),
    PrimitiveTypes::Dbl => Value::Array(cbrtv_f64in(&values)),
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
pub fn cbrtv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let value = stack.last().cloned().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "CBRTV",
  })?;
  let result = cbrtv_values(value, num_type, ip)?;
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
  fn supports_float_directives() {
    for num_type in [
      PrimitiveTypes::Hlf,
      PrimitiveTypes::Flt,
      PrimitiveTypes::Dbl,
    ] {
      assert!(cbrtv_values(array(vec![Value::Int32(1)]), num_type, 0).is_ok());
    }
  }

  #[test]
  fn validates_operands_and_preserves_stack() {
    for value in [Value::Bool(false), array(vec![Value::Bool(false)])] {
      let mut stack = Stack::from_vec(vec![value]);
      let original = stack.clone();
      assert!(cbrtv_func(&mut stack, PrimitiveTypes::Flt, 17).is_err());
      assert_eq!(stack, original);
    }
    assert!(cbrtv_values(array(vec![Value::Float32(1.0)]), PrimitiveTypes::Int, 18).is_err());
  }

  #[test]
  fn preserves_nan_behavior() {
    let result = cbrtv_values(
      array(vec![Value::Float32(f32::NAN)]),
      PrimitiveTypes::Flt,
      19,
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
      cbrtv_func(&mut stack, PrimitiveTypes::Flt, 20),
      Err(VMError::StackUnderflow {
        ip: 20,
        opcode: "CBRTV"
      })
    ));
    assert_eq!(stack, original);
  }
}
