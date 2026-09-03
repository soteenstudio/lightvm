/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::neg::neg_f32in::neg_f32in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn negv_f32in(a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .map(|a| Value::Float32(neg_f32in(a.as_f32())))
      .collect(),
  )
}
