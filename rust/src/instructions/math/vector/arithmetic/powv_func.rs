/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::powv::{
  powv_i16in::powv_i16in, powv_i32in::powv_i32in, powv_i64in::powv_i64in, powv_i128in::powv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;

#[inline(always)]
pub fn powv_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
) -> Result<Value, &'static str> {
  let arr_a = match a_val.as_array() {
    Some(v) => v,
    None => return Ok(Value::NaN),
  };
  let arr_b = match b_val.as_array() {
    Some(v) => v,
    None => return Ok(Value::NaN),
  };
  if arr_a.len() != arr_b.len() {
    return Ok(Value::NaN);
  }
  if num_type == PrimitiveTypes::Str {
    return Ok(Value::NaN);
  }
  for x in arr_a.iter().chain(arr_b.iter()) {
    if !x.is_number() {
      return Err(x.type_of());
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(powv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(powv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(powv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(powv_i128in(&arr_a, &arr_b)),
    _ => Value::NaN,
  })
}

#[inline]
pub fn powv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "POWV" });
  }
  let result = powv_values(
    stack[stack.len() - 2].clone(),
    stack[stack.len() - 1].clone(),
    num_type,
  )
  .map_err(|found| VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type),
    found,
  })?;
  stack.pop();
  *stack.last_mut().unwrap() = result;
  Ok(())
}

fn expected_type(num_type: PrimitiveTypes) -> &'static str {
  match num_type {
    PrimitiveTypes::Sht => "Int16",
    PrimitiveTypes::Int => "Int32",
    PrimitiveTypes::Lng => "Int64",
    PrimitiveTypes::Oct => "Int128",
    PrimitiveTypes::Hlf => "Float16",
    PrimitiveTypes::Flt => "Float32",
    PrimitiveTypes::Dbl => "Float64",
    PrimitiveTypes::Str => "String",
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;

  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }

  #[test]
  fn powv_i32_works() {
    let result = powv_values(
      array(vec![Value::Int32(2), Value::Int32(3)]),
      array(vec![Value::Int32(3), Value::Int32(2)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Ok(array(vec![Value::Int32(8), Value::Int32(9)])));
  }

  #[test]
  fn powv_rejects_non_numeric_and_mixed_or_invalid_lengths() {
    let result = powv_values(
      array(vec![Value::Int32(2)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Err("string"));

    let result = powv_values(
      array(vec![Value::Int32(2)]),
      array(vec![Value::Int64(3)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Ok(array(vec![Value::Int32(8)])));

    let result = powv_values(
      array(vec![Value::Int32(2), Value::Int32(3)]),
      array(vec![Value::Int32(3)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Ok(Value::NaN));
  }

  #[test]
  fn powv_reports_element_type_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(2)]),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      powv_func(&mut stack, PrimitiveTypes::Int, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }

  #[test]
  fn powv_non_array_remains_nan() {
    assert_eq!(
      powv_values(Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
  }
}
