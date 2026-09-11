/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::logarithm::ln::ln_f64in::ln_f64in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn lnv_f64in(values: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    values
      .iter()
      .map(|value| Value::Float64(ln_f64in(value.as_f64())))
      .collect(),
  )
}
