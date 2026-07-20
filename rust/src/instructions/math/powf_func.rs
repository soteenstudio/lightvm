/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::powf::{
  powf_f16in::powf_f16in, powf_f32in::powf_f32in, powf_f64in::powf_f64in,
};
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn powf_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Hlf => Value::Float16(powf_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(powf_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(powf_f64in(a.as_f64(), b.as_f64())),
    _ => Value::NaN,
  }
}
#[inline]
pub fn powf_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "POWF" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "POWF" })?;
  let a = std::mem::take(a_ref);
  *a_ref = powf_values(a, b, num_type);
  Ok(())
}
