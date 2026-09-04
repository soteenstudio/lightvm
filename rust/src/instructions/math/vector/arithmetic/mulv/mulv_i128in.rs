/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::mul::mul_i128in::mul_i128in;
use crate::types::value::Value;
use std::sync::Arc;
pub fn mulv_i128in(a: &Arc<Vec<Value>>, b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| Value::Int128(mul_i128in(a.as_i128(), b.as_i128())))
      .collect(),
  )
}
