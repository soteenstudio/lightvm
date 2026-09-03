use crate::instructions::math::arithmetic::div::div_f32in::div_f32in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn divv_f32in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Float32(div_f32in(a.as_f32(), b.as_f32())))
      .collect(),
  )
}
/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */
