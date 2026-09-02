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
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(dot_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Int32(dot_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Int64(dot_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Int128(dot_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Float16(dot_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Float32(dot_f32in(&arr_a, &arr_b)),
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
