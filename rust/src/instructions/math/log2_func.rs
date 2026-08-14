/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::log2::{
  log2_f16in::log2_f16in, log2_f32in::log2_f32in, log2_f64in::log2_f64in,
};
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn log2_values(a: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Hlf => Value::Float16(log2_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(log2_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(log2_f64in(a.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn log2_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "LOG2" })?;
  let val = std::mem::take(val_ref);
  *val_ref = log2_values(val, num_type);
  Ok(())
}
