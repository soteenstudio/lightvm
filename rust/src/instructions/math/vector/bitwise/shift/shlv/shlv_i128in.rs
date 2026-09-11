/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::bitwise::shift::shl::shl_i128in::shl_i128in;
use crate::types::value::Value;
use std::sync::Arc;
pub fn shlv_i128in(left: &Arc<Vec<Value>>, right: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    left
      .iter()
      .zip(right.iter())
      .map(|(left, right)| Value::Int128(shl_i128in(left.as_i128(), right.as_i128())))
      .collect(),
  )
}
