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
use std::sync::Arc;
pub fn dot_i16in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> i16 {
  let mut sum: i16 = 0;
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let vx: i16 = x.as_i16();
    let vy: i16 = y.as_i16();
    sum = sum.wrapping_add(vx.wrapping_mul(vy));
  }
  sum
}
