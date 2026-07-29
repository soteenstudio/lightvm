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
pub fn specialized_instructions(bytecode: &mut [Instructions]) {
  for instr in bytecode.iter_mut() {
    if let Instructions::Push(val) = instr {
      let replacement = match val {
        Value::Int16(v) => Some(Instructions::PushInt16(*v)),
        Value::Int32(v) => Some(Instructions::PushInt32(*v)),
        Value::Int64(v) => Some(Instructions::PushInt64(*v)),
        Value::Int128(v) => Some(Instructions::PushInt128(*v)),
        Value::Float16(v) => Some(Instructions::PushFloat16(*v)),
        Value::Float32(v) => Some(Instructions::PushFloat32(*v)),
        Value::Float64(v) => Some(Instructions::PushFloat64(*v)),
        Value::String(v) => Some(Instructions::PushString(v.clone())),
        Value::Bool(v) => Some(Instructions::PushBool(*v)),
        Value::Array(v) => Some(Instructions::PushArray(v.clone())),
        Value::Object(v) => Some(Instructions::PushObject(v.clone())),
        Value::Undefined => Some(Instructions::PushUndefined),
        Value::Null => Some(Instructions::PushNull),
        Value::NaN => Some(Instructions::PushNaN),
        _ => None,
      };
      if let Some(new_instr) = replacement {
        *instr = new_instr;
      }
    }
  }
}
