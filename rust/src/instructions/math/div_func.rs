/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::math::div::{
  div_f16in::div_f16in, div_f32in::div_f32in, div_f64in::div_f64in, div_i128in::div_i128in,
  div_i16in::div_i16in, div_i32in::div_i32in, div_i64in::div_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn div_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(div_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(div_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(div_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(div_i128in(a.as_i128(), b.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(div_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(div_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(div_f64in(a.as_f64(), b.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn div_func(
  stack: &mut Vec<Value>,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DIV" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DIV" })?;
  let a = std::mem::take(a_ref);
  *a_ref = div_values(a, b, num_type);
  Ok(())
}
