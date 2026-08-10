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
pub fn extract_value(instr: &Instructions) -> Option<Value> {
  match instr {
    Instructions::PushInt16(v) => Some(Value::Int16(*v)),
    Instructions::PushInt32(v) => Some(Value::Int32(*v)),
    Instructions::PushInt64(v) => Some(Value::Int64(*v)),
    Instructions::PushInt128(v) => Some(Value::Int128(*v)),
    Instructions::PushFloat16(v) => Some(Value::Float16(*v)),
    Instructions::PushFloat32(v) => Some(Value::Float32(*v)),
    Instructions::PushFloat64(v) => Some(Value::Float64(*v)),
    Instructions::PushString(v) => Some(Value::String(v.clone())),
    Instructions::PushArray(v) => Some(Value::Array(v.clone())),
    Instructions::PushObject(v) => Some(Value::Object(v.clone())),
    Instructions::PushBool(v) => Some(Value::Bool(*v)),
    Instructions::PushNull => Some(Value::Null),
    Instructions::PushUndefined => Some(Value::Undefined),
    Instructions::PushNaN => Some(Value::NaN),
    Instructions::Push(v) => Some(v.clone()),
    _ => None,
  }
}
