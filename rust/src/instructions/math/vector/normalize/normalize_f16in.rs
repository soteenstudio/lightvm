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
use half::f16;
use std::sync::Arc;
pub fn normalize_f16in(values: &[Value]) -> Value {
  let magnitude = values
    .iter()
    .fold(0.0_f32, |magnitude, value| magnitude.hypot(value.as_f32()));
  Value::Array(Arc::new(
    values
      .iter()
      .map(|value| {
        Value::Float16(if magnitude == 0.0 {
          f16::ZERO
        } else {
          f16::from_f32(value.as_f32() / magnitude)
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
    let result = normalize_f16in(&[
      Value::Float16(f16::from_f32(3.0)),
      Value::Float16(f16::from_f32(4.0)),
    ]);
    let values = result.as_array().unwrap();
    assert_eq!(values[0], Value::Float16(f16::from_f32(0.6)));
    assert_eq!(values[1], Value::Float16(f16::from_f32(0.8)));
    assert_eq!(
      normalize_f16in(&[Value::Float16(f16::ZERO)]),
      Value::Array(Arc::new(vec![Value::Float16(f16::ZERO)]))
    );
  }
}
