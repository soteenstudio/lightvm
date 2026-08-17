/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::trigonometry::inverse::atan2::{
  atan2_f16in::atan2_f16in, atan2_f32in::atan2_f32in, atan2_f64in::atan2_f64in,
};
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn atan2_values(y: Value, x: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Hlf => Value::Float16(atan2_f16in(y.as_f16(), x.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(atan2_f32in(y.as_f32(), x.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(atan2_f64in(y.as_f64(), x.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn atan2_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let x_ref = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "ATAN2",
  })?;
  let y_ref = stack.last_mut().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "ATAN2",
  })?;
  let y = std::mem::take(y_ref);
  *y_ref = atan2_values(y, x_ref, num_type);
  Ok(())
}
