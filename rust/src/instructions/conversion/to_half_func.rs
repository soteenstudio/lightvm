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
use half::f16;
pub fn to_half_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let num = match val {
      Value::Float16(i) => Value::Float16(i),
      Value::Float32(f) => Value::Float16(f16::from_f32(f).to_bits()),
      Value::Float64(f) => Value::Float16(f16::from_f64(f).to_bits()),
      Value::Int16(i) => Value::Float16(f16::from_f32(i as f32).to_bits()),
      Value::Int32(i) => Value::Float16(f16::from_f32(i as f32).to_bits()),
      Value::Int64(i) => Value::Float16(f16::from_f32(i as f32).to_bits()),
      Value::String(s) => {
        let f = s.parse::<f32>().unwrap_or(0.0);
        Value::Float16(f16::from_f32(f).to_bits())
      }
      Value::Bool(b) => {
        let f = if b { 1.0 } else { 0.0 };
        Value::Float16(f16::from_f32(f).to_bits())
      }
      _ => Value::Float16(0),
    };
    stack.push(num);
  }
}
