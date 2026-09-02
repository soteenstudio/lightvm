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
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn dot_values(
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
    PrimitiveTypes::Sht => Value::Int16(dot_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Int32(dot_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Int64(dot_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => dot_i128in(&arr_a, &arr_b),
    PrimitiveTypes::Hlf => Value::Float16(dot_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => dot_f32in(&arr_a, &arr_b),
    PrimitiveTypes::Dbl => Value::Float64(dot_f64in(&arr_a, &arr_b)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn dot_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "DOT" });
  }
  let result = dot_values(
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
  fn dot_i128_preserves_valid_values_and_rejects_non_numeric_elements() {
    let result = dot_values(
      array(vec![Value::Int128(5), Value::Int128(6), Value::Int128(7)]),
      array(vec![Value::Int128(8), Value::Int128(9), Value::Int128(10)]),
      PrimitiveTypes::Oct,
    );
    assert_eq!(result, Ok(Value::Int128(164)));
    let result = dot_values(
      array(vec![Value::Int128(5)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Oct,
    );
    assert_eq!(result, Err("string"));
  }
  #[test]
  fn dot_f32_rejects_non_numeric_elements() {
    let result = dot_values(
      array(vec![Value::Float32(5.0)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Flt,
    );
    assert_eq!(result, Err("string"));
  }
  #[test]
  fn dot_reports_element_type_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::Bool(true)]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      dot_func(&mut stack, PrimitiveTypes::Int, 12),
      Err(VMError::TypeMismatch {
        ip: 12,
        expected: "Int32",
        found: "bool"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn dot_structural_errors_remain_nan() {
    assert_eq!(
      dot_values(Value::Int32(1), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
    assert_eq!(
      dot_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int
      ),
      Ok(Value::NaN)
    );
  }
}
