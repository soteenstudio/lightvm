/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::r#mod::mod_f64in::mod_f64in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn modv_f64in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Float64(mod_f64in(a.as_f64(), b.as_f64())))
      .collect(),
  )
}
