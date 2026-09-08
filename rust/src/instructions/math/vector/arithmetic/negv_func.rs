/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::negv::{
  negv_f16in::negv_f16in, negv_f32in::negv_f32in, negv_f64in::negv_f64in, negv_i16in::negv_i16in,
  negv_i32in::negv_i32in, negv_i64in::negv_i64in, negv_i128in::negv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn negv_values(a_val: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  let arr_a = a_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::All),
    found: get_type_name(a_val.clone()),
  })?;
  for value in arr_a.iter() {
    if !value.is_number() {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::All),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(negv_i16in(&arr_a)),
    PrimitiveTypes::Int => Value::Array(negv_i32in(&arr_a)),
    PrimitiveTypes::Lng => Value::Array(negv_i64in(&arr_a)),
    PrimitiveTypes::Oct => Value::Array(negv_i128in(&arr_a)),
    PrimitiveTypes::Hlf => Value::Array(negv_f16in(&arr_a)),
    PrimitiveTypes::Flt => Value::Array(negv_f32in(&arr_a)),
    PrimitiveTypes::Dbl => Value::Array(negv_f64in(&arr_a)),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::All),
        found: num_type.directive(),
      });
    }
  })
}
#[inline]
pub fn negv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let value = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEGV" })?;
  let result = negv_values(value, num_type, ip)?;
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
      negv_func(&mut stack, PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch { ip: 17, .. })
    ));
    assert_eq!(stack, original);
  }

  #[test]
  fn validates_elements_and_directives() {
    assert!(negv_values(array(vec![Value::Int32(1)]), PrimitiveTypes::Int, 11).is_ok());
    assert!(matches!(
      negv_values(array(vec![Value::Bool(false)]), PrimitiveTypes::Int, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        found: "Boolean",
        ..
      })
    ));
    assert!(matches!(
      negv_values(array(vec![Value::Int32(1)]), PrimitiveTypes::Str, 20),
      Err(VMError::TypeMismatch { ip: 20, .. })
    ));
  }

  #[test]
  fn underflow_preserves_stack() {
    let mut stack = Stack::new();
    let original = stack.clone();
    assert!(matches!(
      negv_func(&mut stack, PrimitiveTypes::Int, 23),
      Err(VMError::StackUnderflow {
        ip: 23,
        opcode: "NEGV"
      })
    ));
    assert_eq!(stack, original);
  }
}
