/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::neg::neg_f64in::neg_f64in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn negv_f64in(a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .map(|a| Value::Float64(neg_f64in(a.as_f64())))
      .collect(),
  )
}
