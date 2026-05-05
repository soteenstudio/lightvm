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
pub fn to_long_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let num = match val {
      Value::Int64(i) => Value::Int64(i),
      Value::Int32(i) => Value::Int64(i as i64),
      Value::Float64(f) => Value::Int64(f as i64),
      Value::String(s) => s
        .parse::<i64>()
        .map(Value::Int64)
        .unwrap_or(Value::Int64(0)),
      Value::Bool(b) => Value::Int64(if b { 1 } else { 0 }),
      _ => Value::Int64(0),
    };
    stack.push(num);
  }
}
