/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::neg::neg_f16in::neg_f16in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn negv_f16in(a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .map(|a| Value::Float16(neg_f16in(a.as_f16())))
      .collect(),
  )
}
