/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{instructions::Instructions, value::Value};
#[inline(always)]
pub fn value_to_instruction(val: Value) -> Instructions {
  match val {
    Value::Int16(v) => Instructions::PushInt16(v),
    Value::Int32(v) => Instructions::PushInt32(v),
    Value::Int64(v) => {
      if v >= i16::MIN as i64 && v <= i16::MAX as i64 {
        Instructions::PushInt16(v as i16)
      } else if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
        Instructions::PushInt32(v as i32)
      } else {
        Instructions::PushInt64(v)
      }
    }
    Value::Int128(v) => {
      if v >= i16::MIN as i128 && v <= i16::MAX as i128 {
        Instructions::PushInt16(v as i16)
      } else if v >= i32::MIN as i128 && v <= i32::MAX as i128 {
        Instructions::PushInt32(v as i32)
      } else if v >= i64::MIN as i128 && v <= i64::MAX as i128 {
        Instructions::PushInt64(v as i64)
      } else {
        Instructions::PushInt128(v)
      }
    }
    Value::Float16(v) => Instructions::PushFloat16(v),
    Value::Float32(v) => Instructions::PushFloat32(v),
    Value::Float64(v) => Instructions::PushFloat64(v),
    Value::String(v) => Instructions::PushString(v),
    Value::Array(v) => Instructions::PushArray(v),
    Value::Object(v) => Instructions::PushObject(v),
    Value::Bool(v) => Instructions::PushBool(v),
    Value::Null => Instructions::PushNull,
    Value::Undefined => Instructions::PushUndefined,
    Value::NaN => Instructions::PushNaN,
    other => Instructions::Push(other),
  }
}
