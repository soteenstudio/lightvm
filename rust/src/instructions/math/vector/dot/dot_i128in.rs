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
pub fn dot_i128in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Value {
  let mut sum: i128 = 0;
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    if !x.is_number() || !y.is_number() {
      return Value::NaN;
    }
    let vx: i128 = x.as_i128();
    let vy: i128 = y.as_i128();
    sum = sum.wrapping_add(vx.wrapping_mul(vy));
  }
  Value::Int128(sum)
}
