/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::trigonometry::inverse::atan::{
  atan_f16in::atan_f16in, atan_f32in::atan_f32in, atan_f64in::atan_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn atan_values(a: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Hlf => Value::Float16(atan_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(atan_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(atan_f64in(a.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn atan_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "ATAN" })?;
  let val = std::mem::take(val_ref);
  *val_ref = atan_values(val, num_type);
  Ok(())
}
