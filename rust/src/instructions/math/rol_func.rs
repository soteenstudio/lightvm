/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::math::rol::{
  rol_i128in::rol_i128in, rol_i16in::rol_i16in, rol_i32in::rol_i32in, rol_i64in::rol_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn rol_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(rol_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(rol_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(rol_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(rol_i128in(a.as_i128(), b.as_i128())),
    _ => Value::NaN,
  }
}
