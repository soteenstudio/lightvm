/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::dot::{dot_i16in::dot_i16in, dot_i32in::dot_i32in};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use half::f16;
#[inline(always)]
pub fn dot_values(a_val: Value, b_val: Value, num_type: PrimitiveTypes) -> Value {
  let arr_a = match a_val.as_array() {
    Some(v) => v,
    None => return Value::NaN,
  };
  let arr_b = match b_val.as_array() {
    Some(v) => v,
    None => return Value::NaN,
  };
  if arr_a.len() != arr_b.len() {
    return Value::NaN;
  }
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(dot_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Int32(dot_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => {
      let mut sum: i64 = 0;
      for (x, y) in arr_a.iter().zip(arr_b.iter()) {
        sum = sum.wrapping_add(x.as_i64().wrapping_mul(y.as_i64()));
      }
      Value::Int64(sum)
    }
    PrimitiveTypes::Oct => {
      let mut sum: i128 = 0;
      for (x, y) in arr_a.iter().zip(arr_b.iter()) {
        sum = sum.wrapping_add(x.as_i128().wrapping_mul(y.as_i128()));
      }
      Value::Int128(sum)
    }
    PrimitiveTypes::Hlf => {
      let mut sum = f16::ZERO;
      for (x, y) in arr_a.iter().zip(arr_b.iter()) {
        let prod = f16::from_f32(x.as_f16().to_f32() * y.as_f16().to_f32());
        sum = f16::from_f32(sum.to_f32() + prod.to_f32());
      }
      Value::Float16(sum)
    }
    PrimitiveTypes::Flt => {
      let mut sum: f32 = 0.0;
      for (x, y) in arr_a.iter().zip(arr_b.iter()) {
        sum += x.as_f32() * y.as_f32();
      }
      Value::Float32(sum)
    }
    PrimitiveTypes::Dbl => {
      let mut sum: f64 = 0.0;
      for (x, y) in arr_a.iter().zip(arr_b.iter()) {
        sum += x.as_f64() * y.as_f64();
      }
      Value::Float64(sum)
    }
    _ => Value::NaN,
  }
}
#[inline]
pub fn dot_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b_val = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DOT" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DOT" })?;
  let a_val = std::mem::take(a_ref);
  *a_ref = dot_values(a_val, b_val, num_type);
  Ok(())
}
