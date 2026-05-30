/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */
use crate::types::value::Value;

use crate::instructions::math::shl::{
  shl_i16in::shl_i16in, shl_i32in::shl_i32in, shl_i64in::shl_i64in, shl_i128in::shl_i128in,
};
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn shl_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(shl_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(shl_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(shl_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(shl_i128in(a.as_i128(), b.as_i128())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn shl_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SHL" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SHL" })?;
  let a = std::mem::take(a_ref);
  *a_ref = shl_values(a, b, num_type);
  Ok(())
}
