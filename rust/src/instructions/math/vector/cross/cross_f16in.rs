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

pub fn cross_f16in(a: &[Value], b: &[Value]) -> Value {
  if !a.iter().chain(b.iter()).all(Value::is_number) {
    return Value::NaN;
  }
  let component = |ai: usize, aj: usize, bi: usize, bj: usize| {
    f16::from_f32(
      a[ai].as_f16().to_f32() * b[bi].as_f16().to_f32()
        - a[aj].as_f16().to_f32() * b[bj].as_f16().to_f32(),
    )
  };
  Value::Array(Arc::new(vec![
    Value::Float16(component(1, 2, 2, 1)),
    Value::Float16(component(2, 0, 0, 2)),
    Value::Float16(component(0, 1, 1, 0)),
  ]))
}
