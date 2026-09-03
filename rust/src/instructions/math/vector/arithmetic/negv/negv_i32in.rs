/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::neg::neg_i32in::neg_i32in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn negv_i32in(a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .map(|a| Value::Int32(neg_i32in(a.as_i32())))
      .collect(),
  )
}
