/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::cos::{
  cos_f16in::cos_f16in, cos_f32in::cos_f32in, cos_f64in::cos_f64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn cos_values(a: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Hlf => Value::Float16(cos_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(cos_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(cos_f64in(a.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn cos_func(
  stack: &mut Vec<Value>,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let val_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "COS" })?;
  let val = std::mem::take(val_ref);
  *val_ref = cos_values(val, num_type);
  Ok(())
}
