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
pub fn atan2v_f32in(arr_y: &Arc<Vec<Value>>, arr_x: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_y.len());
  for (y, x) in arr_y.iter().zip(arr_x.iter()) {
    let atan2 = y.as_f32().atan2(x.as_f32());
    res.push(Value::Float32(atan2));
  }
  Arc::new(res)
}
