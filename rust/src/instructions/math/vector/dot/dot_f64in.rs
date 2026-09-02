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
pub fn dot_f64in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> f64 {
  let mut sum: f64 = 0.0;
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let vx: f64 = x.as_f64();
    let vy: f64 = y.as_f64();
    sum += vx * vy;
  }
  sum
}
