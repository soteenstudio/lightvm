/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::powi::{
  powi_f16in::powi_f16in, powi_f32in::powi_f32in, powi_f64in::powi_f64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn powi_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Hlf => Value::Float16(powi_f16in(a.as_f16(), b.as_i16())),
    PrimitiveTypes::Flt => Value::Float32(powi_f32in(a.as_f32(), b.as_i32())),
    PrimitiveTypes::Dbl => Value::Float64(powi_f64in(a.as_f64(), b.as_i32())),
    _ => Value::Undefined,
  }
}
