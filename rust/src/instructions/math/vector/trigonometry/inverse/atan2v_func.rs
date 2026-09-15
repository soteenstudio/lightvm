/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::trigonometry::inverse::atan2v::{
  atan2v_f16in::atan2v_f16in, atan2v_f32in::atan2v_f32in, atan2v_f64in::atan2v_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn atan2v_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  let arr_a = a_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::Float),
    found: get_type_name(a_val.clone()),
  })?;
  let arr_b = b_val.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::Float),
    found: get_type_name(b_val.clone()),
  })?;
  if arr_a.len() != arr_b.len() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: "Array",
    });
  }
  for value in arr_a.iter().chain(arr_b.iter()) {
    if !matches!(
      value,
      Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
    ) {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Array(atan2v_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(atan2v_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(atan2v_f64in(&arr_a, &arr_b)),
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
pub fn atan2v_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "ATAN2V",
    });
  }
  let result = atan2v_values(
    stack.last().unwrap().clone(),
    stack[stack.len() - 2].clone(),
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
  fn validates_float_elements_and_preserves_stack() {
    for (invalid, found) in [
      (Value::Int16(1), "Short"),
      (Value::Int32(1), "Integer"),
      (Value::Int64(1), "Long"),
      (Value::Int128(1), "Octa"),
      (Value::Bool(false), "Boolean"),
    ] {
      for (left, right) in [
        (
          array(vec![invalid.clone()]),
          array(vec![Value::Float32(1.0)]),
        ),
        (
          array(vec![Value::Float32(1.0)]),
          array(vec![invalid.clone()]),
        ),
      ] {
        assert!(matches!(
          atan2v_values(left.clone(), right.clone(), PrimitiveTypes::Flt, 24),
          Err(VMError::TypeMismatch { ip: 24, expected: "Float", found: actual })
            if actual == found
        ));
        let mut stack = Stack::from_vec(vec![left, right]);
        let original = stack.clone();
        assert!(matches!(
          atan2v_func(&mut stack, PrimitiveTypes::Flt, 25),
          Err(VMError::TypeMismatch { ip: 25, expected: "Float", found: actual })
            if actual == found
        ));
        assert_eq!(stack, original);
      }
    }
    for (left, right, num_type, expected) in [
      (
        Value::Float16(half::f16::ONE),
        Value::Float64(1.0),
        PrimitiveTypes::Flt,
        "Float",
      ),
      (
        Value::Float32(1.0),
        Value::Float16(half::f16::ONE),
        PrimitiveTypes::Dbl,
        "Double",
      ),
      (
        Value::Float64(1.0),
        Value::Float32(1.0),
        PrimitiveTypes::Hlf,
        "Half",
      ),
    ] {
      let result = atan2v_values(array(vec![left]), array(vec![right]), num_type, 26).unwrap();
      let elements = result.as_array().unwrap();
      assert_eq!(get_type_name(elements[0].clone()), expected);
    }
    let left = array(vec![Value::Float32(1.0)]);
    let right = array(vec![Value::Float32(1.0)]);
    assert!(matches!(
      atan2v_values(left.clone(), right.clone(), PrimitiveTypes::Int, 27),
      Err(VMError::TypeMismatch {
        ip: 27,
        found: "unknown",
        ..
      })
    ));
    let mut stack = Stack::from_vec(vec![left, right]);
    let original = stack.clone();
    assert!(matches!(
      atan2v_func(&mut stack, PrimitiveTypes::Int, 28),
      Err(VMError::TypeMismatch {
        ip: 28,
        found: "unknown",
        ..
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn reports_type_mismatch_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![Value::Bool(false), array(vec![Value::Int32(1)])]);
    let original = stack.clone();
    assert!(matches!(
      atan2v_func(&mut stack, PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch { ip: 17, .. })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn validates_elements_and_directives() {
    assert!(
      atan2v_values(
        array(vec![Value::Float32(1.0)]),
        array(vec![Value::Float32(1.0)]),
        PrimitiveTypes::Flt,
        11
      )
      .is_ok()
    );
    assert!(matches!(
      atan2v_values(
        array(vec![Value::Bool(false)]),
        array(vec![Value::Float32(1.0)]),
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
      atan2v_values(
        array(vec![Value::Float32(1.0)]),
        array(vec![Value::Float32(1.0)]),
        PrimitiveTypes::Str,
        20
      ),
      Err(VMError::TypeMismatch { ip: 20, .. })
    ));
  }
  #[test]
  fn rejects_invalid_vector_lengths() {
    assert!(matches!(
      atan2v_values(
        array(vec![Value::Float32(1.0)]),
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
      atan2v_func(&mut stack, PrimitiveTypes::Flt, 23),
      Err(VMError::StackUnderflow {
        ip: 23,
        opcode: "ATAN2V"
      })
    ));
    assert_eq!(stack, original);
  }
}
