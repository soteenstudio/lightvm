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
pub fn to_double_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let num = match val {
      Value::Float64(f) => Value::Float64(f),
      Value::Float16(i) => {
        let f = f16::from_bits(i);
        Value::Float64(f.to_f64())
      }
      Value::Int16(i) => Value::Float64(i as f64),
      Value::Int64(i) => Value::Float64(i as f64),
      Value::Float32(f) => Value::Float64(f as f64),
      Value::Int32(i) => Value::Float64(i as f64),
      Value::String(s) => s
        .parse::<f64>()
        .map(Value::Float64)
        .unwrap_or(Value::Float64(0.0)),
      _ => Value::Float64(0.0),
    };
    stack.push(num);
  }
}
