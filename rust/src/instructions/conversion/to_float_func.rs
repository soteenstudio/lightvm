/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
pub fn to_float_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let num = match val {
      Value::Float32(f) => Value::Float32(f),
      Value::Int32(i) => Value::Float32(i as f32),
      Value::Float64(f) => Value::Float32(f as f32),
      Value::Int64(i) => Value::Float32(i as f32),
      Value::String(s) => s
        .parse::<f32>()
        .map(Value::Float32)
        .unwrap_or(Value::Float32(0.0)),
      _ => Value::Float32(0.0),
    };
    stack.push(num);
  }
}
