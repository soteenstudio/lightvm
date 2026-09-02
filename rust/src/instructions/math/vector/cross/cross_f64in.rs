/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use std::sync::Arc;

pub fn cross_f64in(a: &[Value], b: &[Value]) -> Value {
  if !a.iter().chain(b.iter()).all(Value::is_number) {
    return Value::NaN;
  }
  let component = |ai: usize, aj: usize, bi: usize, bj: usize| {
    a[ai].as_f64() * b[bi].as_f64() - a[aj].as_f64() * b[bj].as_f64()
  };
  Value::Array(Arc::new(vec![
    Value::Float64(component(1, 2, 2, 1)),
    Value::Float64(component(2, 0, 0, 2)),
    Value::Float64(component(0, 1, 1, 0)),
  ]))
}
