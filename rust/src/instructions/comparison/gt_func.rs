/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::comparison::gt::{
  gt_f16in::gt_f16in, gt_f32in::gt_f32in, gt_f64in::gt_f64in, gt_i16in::gt_i16in,
  gt_i32in::gt_i32in, gt_i64in::gt_i64in,
};
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn gt_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Bool(gt_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Bool(gt_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(gt_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(gt_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(gt_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Hlf => Value::Bool(gt_f16in(a.as_f32(), b.as_f32())),
    _ => Value::Bool(false),
  }
}
#[inline]
pub fn gt_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "GT" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "GT" })?;
  let a = std::mem::take(a_ref);
  *a_ref = gt_values(a, b, num_type);
  Ok(())
}
