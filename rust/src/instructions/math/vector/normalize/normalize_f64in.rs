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
pub fn normalize_f64in(values: &[Value]) -> Value {
  let magnitude = values
    .iter()
    .fold(0.0_f64, |magnitude, value| magnitude.hypot(value.as_f64()));
  Value::Array(Arc::new(
    values
      .iter()
      .map(|value| {
        Value::Float64(if magnitude == 0.0 {
          0.0
        } else {
          value.as_f64() / magnitude
        })
      })
      .collect(),
  ))
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn normalizes_vectors_and_preserves_zero_vectors() {
    let result = normalize_f64in(&[Value::Float64(3.0), Value::Float64(4.0)]);
    let values = result.as_array().unwrap();
    assert!((values[0].as_f64() - 0.6).abs() < f64::EPSILON);
    assert!((values[1].as_f64() - 0.8).abs() < f64::EPSILON);
    assert_eq!(
      normalize_f64in(&[Value::Float64(0.0)]),
      Value::Array(Arc::new(vec![Value::Float64(0.0)]))
    );
  }
}
