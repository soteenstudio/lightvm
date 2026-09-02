/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use half::f16;
use std::sync::Arc;
pub fn dot_f16in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> f16 {
  let mut sum = f16::ZERO;
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let vx = x.as_f16();
    let vy = y.as_f16();
    sum = f16::from_f32(sum.to_f32() + vx.to_f32() * vy.to_f32());
  }
  sum
}
