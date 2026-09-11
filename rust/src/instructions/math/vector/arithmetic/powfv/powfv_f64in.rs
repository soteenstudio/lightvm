/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::powf::powf_f64in::powf_f64in;
use crate::types::value::Value;
use std::sync::Arc;
pub fn powfv_f64in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  Arc::new(
    arr_a
      .iter()
      .zip(arr_b.iter())
      .map(|(base, exp)| Value::Float64(powf_f64in(base.as_f64(), exp.as_f64())))
      .collect(),
  )
}
