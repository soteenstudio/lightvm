/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::comparison::le::{
  le_f16in::le_f16in, le_f32in::le_f32in, le_f64in::le_f64in, le_i16in::le_i16in,
  le_i32in::le_i32in, le_i64in::le_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn le_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Bool(le_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Bool(le_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(le_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(le_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(le_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Hlf => Value::Bool(le_f16in(a.as_f32(), b.as_f32())),
    _ => Value::Bool(false),
  }
}
