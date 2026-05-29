/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::shr::{
  shr_i16in::shr_i16in, shr_i32in::shr_i32in, shr_i64in::shr_i64in, shr_i128in::shr_i128in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn shr_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(shr_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(shr_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(shr_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(shr_i128in(a.as_i128(), b.as_i128())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn shr_func(
  stack: &mut SmallVec<[Value; 16]>,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SHR" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SHR" })?;
  let a = std::mem::take(a_ref);
  *a_ref = shr_values(a, b, num_type);
  Ok(())
}
