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
pub fn to_integer_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let num = match val {
      Value::Int32(i) => Value::Int32(i),
      Value::Int16(i) => Value::Int32(i as i32),
      Value::Int64(i) => Value::Int32(i as i32),
      Value::Float16(i) => {
        let f = f16::from_bits(i);
        Value::Int32(f.to_f32() as i32)
      }
      Value::Float32(f) => Value::Int32(f as i32),
      Value::Float64(f) => Value::Int32(f as i32),
      Value::String(s) => s
        .parse::<i32>()
        .map(Value::Int32)
        .unwrap_or(Value::Int32(0)),
      Value::Bool(b) => Value::Int32(if b { 1 } else { 0 }),
      _ => Value::Int32(0),
    };
    stack.push(num);
  }
}
