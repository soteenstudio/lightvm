/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::r#mod::mod_f32in::mod_f32in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn modv_f32in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Float32(mod_f32in(a.as_f32(), b.as_f32())))
      .collect(),
  )
}
