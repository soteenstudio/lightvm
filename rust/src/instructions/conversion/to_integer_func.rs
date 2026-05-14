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
#[inline(always)]
pub fn to_integer_func(stack: &mut [Value]) {
  if let Some(top) = stack.last_mut() {
    let num = match top {
      Value::String(s) => s.parse::<i32>().unwrap_or(0),
      _ => top.as_i32(),
    };
    *top = Value::Int32(num);
  }
}
