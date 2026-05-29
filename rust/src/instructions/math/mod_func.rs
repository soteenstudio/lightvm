/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::r#mod::{
  mod_f16in::mod_f16in, mod_f32in::mod_f32in, mod_f64in::mod_f64in, mod_i16in::mod_i16in,
  mod_i32in::mod_i32in, mod_i64in::mod_i64in, mod_i128in::mod_i128in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn mod_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(mod_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(mod_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(mod_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(mod_i128in(a.as_i128(), b.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(mod_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(mod_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(mod_f64in(a.as_f64(), b.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn mod_func(
  stack: &mut SmallVec<[Value; 16]>,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "MOD" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "MOD" })?;
  let a = std::mem::take(a_ref);
  *a_ref = mod_values(a, b, num_type);
  Ok(())
}
