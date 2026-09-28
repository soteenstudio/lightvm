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
pub fn dot_f64in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Result<f64, Value> {
  let mut sum: f64 = 0.0;
  let mut invalid_a = None;
  let mut invalid_b = None;
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let x_valid = matches!(x, Value::Float16(_) | Value::Float32(_) | Value::Float64(_));
    let y_valid = matches!(y, Value::Float16(_) | Value::Float32(_) | Value::Float64(_));
    if !x_valid {
      invalid_a.get_or_insert_with(|| x.clone());
    }
    if !y_valid {
      invalid_b.get_or_insert_with(|| y.clone());
    }
    if !x_valid || !y_valid {
      continue;
    }
    let vx: f64 = x.as_f64();
    let vy: f64 = y.as_f64();
    sum += vx * vy;
  }
  invalid_a.or(invalid_b).map_or(Ok(sum), Err)
}
