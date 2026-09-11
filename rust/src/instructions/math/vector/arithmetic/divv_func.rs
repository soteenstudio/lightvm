/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::divv::{
  divv_f16in::divv_f16in, divv_f32in::divv_f32in, divv_f64in::divv_f64in, divv_i16in::divv_i16in,
  divv_i32in::divv_i32in, divv_i64in::divv_i64in, divv_i128in::divv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn divv_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  let arr_a = a_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::All),
    found: get_type_name(a_val.clone()),
  })?;
  let arr_b = b_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::All),
    found: get_type_name(b_val.clone()),
  })?;
  if arr_a.len() != arr_b.len() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::All),
      found: "Array",
    });
  }
  for value in arr_a.iter().chain(arr_b.iter()) {
    if !value.is_number() {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::All),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(divv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(divv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(divv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(divv_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Array(divv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(divv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(divv_f64in(&arr_a, &arr_b)),
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
pub fn divv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "DIVV" });
  }
  let result = divv_values(
    stack[stack.len() - 2].clone(),
    stack.last().unwrap().clone(),
    num_type,
    ip,
  )?;
  stack.pop();
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
    let mut stack = Stack::from_vec(vec![Value::Bool(false), array(vec![Value::Int32(1)])]);
    let original = stack.clone();
    assert!(matches!(
      divv_func(&mut stack, PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch { ip: 17, .. })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn validates_elements_and_directives() {
    assert!(
      divv_values(
        array(vec![Value::Int32(1)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Int,
        11
      )
      .is_ok()
    );
    assert!(matches!(
      divv_values(
        array(vec![Value::Bool(false)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Int,
        19
      ),
      Err(VMError::TypeMismatch {
        ip: 19,
        found: "Boolean",
        ..
      })
    ));
    assert!(matches!(
      divv_values(
        array(vec![Value::Int32(1)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Str,
        20
      ),
      Err(VMError::TypeMismatch { ip: 20, .. })
    ));
  }
  #[test]
  fn rejects_invalid_vector_lengths() {
    assert!(matches!(
      divv_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int,
        21,
      ),
      Err(VMError::TypeMismatch {
        ip: 21,
        found: "Array",
        ..
      })
    ));
  }
  #[test]
  fn underflow_preserves_stack() {
    let mut stack = Stack::from_vec(vec![array(vec![])]);
    let original = stack.clone();
    assert!(matches!(
      divv_func(&mut stack, PrimitiveTypes::Int, 23),
      Err(VMError::StackUnderflow {
        ip: 23,
        opcode: "DIVV"
      })
    ));
    assert_eq!(stack, original);
  }
}
