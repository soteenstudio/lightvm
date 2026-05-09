/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::ror::{
  ror_i16in::ror_i16in, ror_i32in::ror_i32in, ror_i64in::ror_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn ror_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Sht => Value::Int16(ror_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(ror_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(ror_i64in(a.as_i64(), b.as_i64())),
    _ => Value::Undefined,
  }
}
