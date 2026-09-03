/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::neg::neg_i64in::neg_i64in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn negv_i64in(a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .map(|a| Value::Int64(neg_i64in(a.as_i64())))
      .collect(),
  )
}
