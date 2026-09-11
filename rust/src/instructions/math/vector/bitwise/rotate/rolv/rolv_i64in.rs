/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::bitwise::rotate::rol::rol_i64in::rol_i64in;
use crate::types::value::Value;
use std::sync::Arc;

pub fn rolv_i64in(left: &Arc<Vec<Value>>, right: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    left
      .iter()
      .zip(right.iter())
      .map(|(left, right)| Value::Int64(rol_i64in(left.as_i64(), right.as_i64())))
      .collect(),
  )
}
