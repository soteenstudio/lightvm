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
pub fn addv_i32in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len().min(arr_b.len()));
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    res.push(Value::Int32(x.as_i32().wrapping_add(y.as_i32())));
  }
  Arc::new(res)
}
