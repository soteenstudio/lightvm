/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::add::{
  add_f32in::add_f32in, add_f64in::add_f64in, add_i16in::add_i16in, add_i32in::add_i32in,
  add_i64in::add_i64in,
};
use crate::instructions::math::sub::{
  sub_f32in::sub_f32in, sub_f64in::sub_f64in, sub_i16in::sub_i16in, sub_i32in::sub_i32in,
  sub_i64in::sub_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn inc_func(
  vars: &mut Vec<Value>,
  stack: &mut Vec<Value>,
  index: usize,
  num_type: PrimitiveTypes,
  is_hot: bool,
) -> Result<(), String> {
  let old_val = vars.get(index).cloned().unwrap_or(Value::Undefined);
  if !old_val.is_number() {
    return Err(format!(
      "TypeError: Cannot increment non-number variable at index '{}'",
      index
    ));
  }
  let result = if is_hot {
    match num_type {
      PrimitiveTypes::Sht => Value::Int16(add_i16in(old_val.as_i16(), 1)),
      PrimitiveTypes::Int => Value::Int32(add_i32in(old_val.as_i32(), 1)),
      PrimitiveTypes::Lng => Value::Int64(add_i64in(old_val.as_i64(), 1)),
      PrimitiveTypes::Flt => Value::Float32(add_f32in(old_val.as_f32(), 1.0)),
      PrimitiveTypes::Dbl => Value::Float64(add_f64in(old_val.as_f64(), 1.0)),
      _ => Value::Int32(add_i32in(old_val.as_i32(), 1)),
    }
  } else {
    Value::Int32(add_i32in(old_val.as_i32(), 1))
  };
  if index < vars.len() {
    vars[index] = result.clone();
  } else {
    vars.resize(index + 1, Value::Undefined);
    vars[index] = result.clone();
  }
  stack.push(result);
  Ok(())
}
#[inline]
pub fn dec_func(
  vars: &mut Vec<Value>,
  index: usize,
  num_type: PrimitiveTypes,
) -> Result<(), String> {
  let old_val = vars.get(index).cloned().unwrap_or(Value::Undefined);
  if !old_val.is_number() {
    return Err(format!(
      "TypeError: Cannot decrement non-number variable at index '{}'",
      index
    ));
  }
  let result = match num_type {
    PrimitiveTypes::Sht => Value::Int16(sub_i16in(old_val.as_i16(), 1)),
    PrimitiveTypes::Int => Value::Int32(sub_i32in(old_val.as_i32(), 1)),
    PrimitiveTypes::Lng => Value::Int64(sub_i64in(old_val.as_i64(), 1)),
    PrimitiveTypes::Flt => Value::Float32(sub_f32in(old_val.as_f32(), 1.0)),
    PrimitiveTypes::Dbl => Value::Float64(sub_f64in(old_val.as_f64(), 1.0)),
    _ => Value::Int32(sub_i32in(old_val.as_i32(), 1)),
  };
  if index < vars.len() {
    vars[index] = result;
  }
  Ok(())
}
