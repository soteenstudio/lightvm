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
use ahash::AHashMap;
use smol_str::SmolStr;
fn extract_push_value(instr: &Instructions) -> Option<Value> {
  match instr {
    Instructions::PushInt16(v) => Some(Value::Int16(*v)),
    Instructions::PushInt32(v) => Some(Value::Int32(*v)),
    Instructions::PushInt64(v) => Some(Value::Int64(*v)),
    Instructions::PushInt128(v) => Some(Value::Int128(*v)),
    Instructions::PushFloat16(v) => Some(Value::Float16(*v)),
    Instructions::PushFloat32(v) => Some(Value::Float32(*v)),
    Instructions::PushFloat64(v) => Some(Value::Float64(*v)),
    Instructions::PushString(v) => Some(Value::String(v.clone())),
    Instructions::PushBool(v) => Some(Value::Bool(*v)),
    Instructions::PushNull => Some(Value::Null),
    Instructions::PushUndefined => Some(Value::Undefined),
    Instructions::PushNaN => Some(Value::NaN),
    Instructions::Push(v) => Some(v.clone()),
    _ => None,
  }
}
pub fn constant_propagation(bytecode: &mut [Instructions]) {
  let mut get_counts: AHashMap<SmolStr, usize> = AHashMap::new();
  for instr in bytecode.iter() {
    if let Instructions::Get(name) = instr {
      *get_counts.entry(name.clone()).or_insert(0) += 1;
    }
  }
  const MAX_INLINE_USES: usize = 8;
  let mut const_map: AHashMap<SmolStr, Value> = AHashMap::new();
  let mut i = 0;
  while i + 1 < bytecode.len() {
    if let Some(val) = extract_push_value(&bytecode[i])
      && let Instructions::Set(ref name) = bytecode[i + 1]
    {
      let usage_count = get_counts.get(name).copied().unwrap_or(0);
      let is_heavy = matches!(val, Value::Array(_) | Value::Object(_));
      if usage_count <= MAX_INLINE_USES && !is_heavy {
        const_map.insert(name.clone(), val);
      }
    }
    i += 1;
  }
  for instr in bytecode.iter_mut() {
    if let Instructions::Get(name) = instr
      && let Some(val) = const_map.get(name)
    {
      *instr = Instructions::Push(val.clone());
    }
  }
}
