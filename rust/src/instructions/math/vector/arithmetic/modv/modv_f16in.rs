/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::r#mod::mod_f16in::mod_f16in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn modv_f16in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Float16(mod_f16in(a.as_f16(), b.as_f16())))
      .collect(),
  )
}
