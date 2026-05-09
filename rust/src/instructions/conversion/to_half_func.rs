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
  if let Some(top) = stack.last_mut() {
    let num = match top {
      Value::String(s) => {
        let f = s.parse::<f32>().unwrap_or(0.0);
        f16::from_f32(f).to_bits()
      }
      _ => top.as_f16(),
    };
    *top = Value::Float16(num);
  }
}
