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
pub fn dot_values(a_val: Value, b_val: Value, num_type: PrimitiveTypes) -> Value {
  let arr_a = match a_val.as_array() {
    Some(v) => v,
    None => return Value::NaN,
  };
  let arr_b = match b_val.as_array() {
    Some(v) => v,
    None => return Value::NaN,
  };
  if arr_a.len() != arr_b.len() {
    return Value::NaN;
  }
  let validator: fn(&Value) -> bool = match num_type {
    PrimitiveTypes::Sht => |v| matches!(v, Value::Int16(_)),
    PrimitiveTypes::Int => |v| matches!(v, Value::Int32(_)),
    PrimitiveTypes::Lng => |v| matches!(v, Value::Int64(_)),
    PrimitiveTypes::Oct => |v| matches!(v, Value::Int128(_)),
    PrimitiveTypes::Hlf => |v| matches!(v, Value::Float16(_)),
    PrimitiveTypes::Flt => |v| matches!(v, Value::Float32(_)),
    PrimitiveTypes::Dbl => |v| matches!(v, Value::Float64(_)),
    _ => return Value::NaN,
  };
  for x in arr_a.iter().chain(arr_b.iter()) {
    if !validator(x) {
      return Value::NaN;
    }
  }
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(dot_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Int32(dot_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Int64(dot_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => dot_i128in(&arr_a, &arr_b),
    PrimitiveTypes::Hlf => Value::Float16(dot_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => dot_f32in(&arr_a, &arr_b),
    PrimitiveTypes::Dbl => Value::Float64(dot_f64in(&arr_a, &arr_b)),
    _ => Value::NaN,
  }
}
#[inline]
pub fn dot_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b_val = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DOT" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DOT" })?;
  let a_val = std::mem::take(a_ref);
  *a_ref = dot_values(a_val, b_val, num_type);
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
  fn dot_i128_preserves_valid_values_and_rejects_non_numeric_elements() {
    let result = dot_values(
      array(vec![Value::Int128(5), Value::Int128(6), Value::Int128(7)]),
      array(vec![Value::Int128(8), Value::Int128(9), Value::Int128(10)]),
      PrimitiveTypes::Oct,
    );
    assert_eq!(result, Value::Int128(164));
    let result = dot_values(
      array(vec![Value::Int128(5)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Oct,
    );
    assert_eq!(result, Value::NaN);
  }
  #[test]
  fn dot_f32_rejects_non_numeric_elements() {
    let result = dot_values(
      array(vec![Value::Float32(5.0)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Flt,
    );
    assert_eq!(result, Value::NaN);
  }
}
