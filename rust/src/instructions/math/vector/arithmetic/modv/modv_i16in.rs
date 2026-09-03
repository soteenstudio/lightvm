/*
 * Copyright 2025-2026 SoTeen Studio
 * Licensed under the Apache License, Version 2.0
 */

use crate::instructions::math::arithmetic::r#mod::mod_i16in::mod_i16in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn modv_i16in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Int16(mod_i16in(a.as_i16(), b.as_i16())))
      .collect(),
  )
}
