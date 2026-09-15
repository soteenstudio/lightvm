/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::dot::{
  dot_f16in::dot_f16in, dot_f32in::dot_f32in, dot_f64in::dot_f64in, dot_i16in::dot_i16in,
  dot_i32in::dot_i32in, dot_i64in::dot_i64in, dot_i128in::dot_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn dot_values(
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
    let is_valid = match num_type {
      PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct => {
        matches!(
          value,
          Value::Int16(_) | Value::Int32(_) | Value::Int64(_) | Value::Int128(_)
        )
      }
      PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl => matches!(
        value,
        Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
      ),
      _ => true,
    };
    if !is_valid {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::All),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Int16(dot_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Int32(dot_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Int64(dot_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => dot_i128in(&arr_a, &arr_b),
    PrimitiveTypes::Hlf => Value::Float16(dot_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => dot_f32in(&arr_a, &arr_b),
    PrimitiveTypes::Dbl => Value::Float64(dot_f64in(&arr_a, &arr_b)),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::All),
        found: "unknown",
      });
    }
  })
}
#[inline]
pub fn dot_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "DOT" });
  }
  let result = dot_values(
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
      dot_func(&mut stack, PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch { ip: 17, .. })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn rejects_mismatched_numeric_families_without_mutating_stack() {
    for (left, right, num_type, expected, found) in [
      (
        array(vec![Value::Int32(1)]),
        array(vec![Value::Float32(1.0)]),
        PrimitiveTypes::Int,
        "Integer",
        "Float",
      ),
      (
        array(vec![Value::Float64(1.0)]),
        array(vec![Value::Int64(1)]),
        PrimitiveTypes::Dbl,
        "Double",
        "Long",
      ),
      (
        array(vec![Value::Int32(1)]),
        array(vec![Value::Bool(false)]),
        PrimitiveTypes::Int,
        "Integer",
        "Boolean",
      ),
    ] {
      let mut stack = Stack::from_vec(vec![left, right]);
      let original = stack.clone();
      assert!(matches!(
        dot_func(&mut stack, num_type, 24),
        Err(VMError::TypeMismatch { ip: 24, expected: actual_expected, found: actual_found })
          if actual_expected == expected && actual_found == found
      ));
      assert_eq!(stack, original);
    }
  }
  #[test]
  fn unsupported_directive_reports_unknown_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::Int32(1)]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      dot_func(&mut stack, PrimitiveTypes::Str, 25),
      Err(VMError::TypeMismatch {
        ip: 25,
        found: "unknown",
        ..
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn validates_elements_and_directives() {
    assert!(
      dot_values(
        array(vec![Value::Int32(1)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Int,
        11
      )
      .is_ok()
    );
    assert!(matches!(
      dot_values(
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
      dot_values(
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
      dot_values(
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
      dot_func(&mut stack, PrimitiveTypes::Int, 23),
      Err(VMError::StackUnderflow {
        ip: 23,
        opcode: "DOT"
      })
    ));
    assert_eq!(stack, original);
  }
}
