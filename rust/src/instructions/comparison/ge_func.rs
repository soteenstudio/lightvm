/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::comparison::ge::{
  ge_f16in::ge_f16in, ge_f32in::ge_f32in, ge_f64in::ge_f64in, ge_i16in::ge_i16in,
  ge_i32in::ge_i32in, ge_i64in::ge_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn ge_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Bool(ge_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Bool(ge_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(ge_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(ge_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(ge_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Hlf => Value::Bool(ge_f16in(a.as_f32(), b.as_f32())),
    _ => Value::Bool(false),
  }
}
#[inline]
pub fn ge_func(
  stack: &mut SmallVec<[Value; 16]>,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "GE" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "GE" })?;
  let a = std::mem::take(a_ref);
  *a_ref = ge_values(a, b, num_type);
  Ok(())
}
