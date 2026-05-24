/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::neg::{
  neg_f16in::neg_f16in, neg_f32in::neg_f32in, neg_f64in::neg_f64in, neg_i128in::neg_i128in,
  neg_i16in::neg_i16in, neg_i32in::neg_i32in, neg_i64in::neg_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn neg_values(a: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(neg_i16in(a.as_i16())),
    PrimitiveTypes::Int => Value::Int32(neg_i32in(a.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(neg_i64in(a.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(neg_i128in(a.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(neg_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(neg_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(neg_f64in(a.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn neg_func(stack: &mut [Value], num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEG" })?;
  let a = std::mem::take(a_ref);
  *a_ref = neg_values(a, num_type);
  Ok(())
}
