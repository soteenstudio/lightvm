use crate::instructions::math::arithmetic::div::div_i128in::div_i128in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn divv_i128in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Int128(div_i128in(a.as_i128(), b.as_i128())))
      .collect(),
  )
}
/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */
