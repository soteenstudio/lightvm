/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::shrv::{
  shrv_i16in::shrv_i16in, shrv_i32in::shrv_i32in, shrv_i64in::shrv_i64in, shrv_i128in::shrv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};

#[inline(always)]
pub fn shrv_values(
  left_value: Value,
  right_value: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  let left = left_value.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::Integer),
    found: get_type_name(left_value.clone()),
  })?;
  let right = right_value.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type, ExpectedCategory::Integer),
    found: get_type_name(right_value.clone()),
  })?;
  if left.len() != right.len() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Integer),
      found: "Array",
    });
  }
  for value in left.iter().chain(right.iter()) {
    if !value.is_number() {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Integer),
        found: get_type_name(value.clone()),
      });
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(shrv_i16in(&left, &right)),
    PrimitiveTypes::Int => Value::Array(shrv_i32in(&left, &right)),
    PrimitiveTypes::Lng => Value::Array(shrv_i64in(&left, &right)),
    PrimitiveTypes::Oct => Value::Array(shrv_i128in(&left, &right)),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Integer),
        found: num_type.directive(),
      });
    }
  })
}

#[inline]
pub fn shrv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "SHRV" });
  }
  let result = shrv_values(
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
  fn supports_integer_directives() {
    for num_type in [
      PrimitiveTypes::Sht,
      PrimitiveTypes::Int,
      PrimitiveTypes::Lng,
      PrimitiveTypes::Oct,
    ] {
      assert!(
        shrv_values(
          array(vec![Value::Int32(1)]),
          array(vec![Value::Int32(1)]),
          num_type,
          0
        )
        .is_ok()
      );
    }
  }

  #[test]
  fn validates_operands_and_preserves_stack() {
    let cases = [
      (Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      (
        array(vec![Value::Bool(false)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Int,
      ),
      (
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int,
      ),
      (
        array(vec![Value::Int32(1)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Flt,
      ),
    ];
    for (left, right, num_type) in cases {
      let mut stack = Stack::from_vec(vec![left, right]);
      let original = stack.clone();
      assert!(shrv_func(&mut stack, num_type, 17).is_err());
      assert_eq!(stack, original);
    }
  }

  #[test]
  fn underflow_preserves_stack() {
    let mut stack = Stack::from_vec(vec![array(vec![])]);
    let original = stack.clone();
    assert!(matches!(
      shrv_func(&mut stack, PrimitiveTypes::Int, 18),
      Err(VMError::StackUnderflow {
        ip: 18,
        opcode: "SHRV"
      })
    ));
    assert_eq!(stack, original);
  }
}
