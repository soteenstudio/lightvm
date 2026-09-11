/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::powiv::{
  powiv_f16in::powiv_f16in, powiv_f32in::powiv_f32in, powiv_f64in::powiv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
fn expected_powiv_type(num_type: PrimitiveTypes) -> &'static str {
  match num_type {
    PrimitiveTypes::Hlf => "Float16/Int16",
    PrimitiveTypes::Flt => "Float32/Int32",
    PrimitiveTypes::Dbl => "Float64/Int64",
    _ => "Float32/Int32",
  }
}
#[inline(always)]
pub fn powiv_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  let arr_a = a_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_powiv_type(num_type),
    found: get_type_name(a_val.clone()),
  })?;
  let arr_b = b_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_powiv_type(num_type),
    found: get_type_name(b_val.clone()),
  })?;
  if arr_a.len() != arr_b.len() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_powiv_type(num_type),
      found: "Array",
    });
  }
  for value in arr_a.iter().chain(arr_b.iter()) {
    if !value.is_number() {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_powiv_type(num_type),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Array(powiv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(powiv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(powiv_f64in(&arr_a, &arr_b)),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_powiv_type(num_type),
        found: expected_type(num_type, ExpectedCategory::All),
      });
    }
  })
}
#[inline]
pub fn powiv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "POWIV",
    });
  }
  let result = powiv_values(
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
      powiv_func(&mut stack, PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch { ip: 17, .. })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn validates_elements_and_directives() {
    assert!(
      powiv_values(
        array(vec![Value::Int32(1)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Flt,
        11
      )
      .is_ok()
    );
    assert!(matches!(
      powiv_values(
        array(vec![Value::Bool(false)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Flt,
        19
      ),
      Err(VMError::TypeMismatch {
        ip: 19,
        found: "Boolean",
        ..
      })
    ));
    assert!(matches!(
      powiv_values(
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
      powiv_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Flt,
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
      powiv_func(&mut stack, PrimitiveTypes::Flt, 23),
      Err(VMError::StackUnderflow {
        ip: 23,
        opcode: "POWIV"
      })
    ));
    assert_eq!(stack, original);
  }
}
