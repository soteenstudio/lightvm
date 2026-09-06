/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::pow::pow_i64in::pow_i64in;
use crate::types::value::Value;
use std::sync::Arc;
pub fn powv_i64in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len().min(arr_b.len()));
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let base = x.as_i64();
    let exp = y.as_i64();
    res.push(Value::Int64(pow_i64in(base, exp)));
  }
  Arc::new(res)
}
