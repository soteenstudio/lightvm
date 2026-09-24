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
pub fn dot_i16in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Result<i16, Value> {
  let mut sum: i16 = 0;
  let mut invalid_a = None;
  let mut invalid_b = None;
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let x_valid = matches!(
      x,
      Value::Int16(_) | Value::Int32(_) | Value::Int64(_) | Value::Int128(_)
    );
    let y_valid = matches!(
      y,
      Value::Int16(_) | Value::Int32(_) | Value::Int64(_) | Value::Int128(_)
    );
    if !x_valid {
      invalid_a.get_or_insert_with(|| x.clone());
    }
    if !y_valid {
      invalid_b.get_or_insert_with(|| y.clone());
    }
    if !x_valid || !y_valid {
      continue;
    }
    let vx: i16 = x.as_i16();
    let vy: i16 = y.as_i16();
    sum = sum.wrapping_add(vx.wrapping_mul(vy));
  }
  invalid_a.or(invalid_b).map_or(Ok(sum), Err)
}
