/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::addv::{
  addv_f16in::addv_f16in, addv_f32in::addv_f32in, addv_f64in::addv_f64in, addv_i16in::addv_i16in,
  addv_i32in::addv_i32in, addv_i64in::addv_i64in, addv_i128in::addv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn addv_values(a_val: Value, b_val: Value, num_type: PrimitiveTypes) -> Value {
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
    PrimitiveTypes::Sht => Value::Array(addv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(addv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(addv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(addv_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Array(addv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(addv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(addv_f64in(&arr_a, &arr_b)),
    _ => Value::NaN,
  }
}
#[inline]
pub fn addv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b_val = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "ADDV" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "ADDV" })?;
  let a_val = std::mem::take(a_ref);
  *a_ref = addv_values(a_val, b_val, num_type);
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
  fn addv_i32_works() {
    let result = addv_values(
      array(vec![Value::Int32(1), Value::Int32(2)]),
      array(vec![Value::Int32(10), Value::Int32(20)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, array(vec![Value::Int32(11), Value::Int32(22)]));
  }
  #[test]
  fn addv_rejects_non_numeric_and_mixed_or_invalid_lengths() {
    let result = addv_values(
      array(vec![Value::Int32(1)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Value::NaN);
    let result = addv_values(
      array(vec![Value::Int32(1)]),
      array(vec![Value::Int64(1)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Value::NaN);
    let result = addv_values(
      array(vec![Value::Int32(1), Value::Int32(2)]),
      array(vec![Value::Int32(1)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Value::NaN);
  }
}
