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
pub fn to_short_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let num = match val {
      Value::Int16(i) => Value::Int16(i),
      Value::Int32(i) => Value::Int16(i as i16),
      Value::Int64(i) => Value::Int16(i as i16),
      Value::Float16(i) => {
        let f = f16::from_bits(i);
        Value::Int16(f.to_f32() as i16)
      }
      Value::Float32(f) => Value::Int16(f as i16),
      Value::Float64(f) => Value::Int16(f as i16),
      Value::String(s) => s
        .parse::<i16>()
        .map(Value::Int16)
        .unwrap_or(Value::Int16(0)),
      Value::Bool(b) => Value::Int16(if b { 1 } else { 0 }),
      _ => Value::Int16(0),
    };
    stack.push(num);
  }
}
