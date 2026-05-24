/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::comparison::lt::{
  lt_f16in::lt_f16in, lt_f32in::lt_f32in, lt_f64in::lt_f64in, lt_i16in::lt_i16in,
  lt_i32in::lt_i32in, lt_i64in::lt_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn lt_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Bool(lt_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Bool(lt_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(lt_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(lt_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(lt_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Hlf => Value::Bool(lt_f16in(a.as_f32(), b.as_f32())),
    _ => Value::Bool(false),
  }
}
#[inline]
pub fn lt_func(stack: &mut Vec<Value>, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "LT" })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "LT" })?;
  let a = std::mem::take(a_ref);
  *a_ref = lt_values(a, b, num_type);
  Ok(())
}
