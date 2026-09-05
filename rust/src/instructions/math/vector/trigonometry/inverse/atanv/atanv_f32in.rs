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
pub fn atanv_f32in(arr_y: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_y.len());
  for y in arr_y.iter() {
    let atan = (y.as_f32() as f64).atan() as f32;
    res.push(Value::Float32(atan));
  }
  Arc::new(res)
}
