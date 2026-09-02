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

pub fn cross_i32in(a: &[Value], b: &[Value]) -> Value {
  if !a.iter().chain(b.iter()).all(Value::is_number) {
    return Value::NaN;
  }
  let component = |ai, aj, bi, bj| {
    a[ai]
      .as_i32()
      .wrapping_mul(b[bi].as_i32())
      .wrapping_sub(a[aj].as_i32().wrapping_mul(b[bj].as_i32()))
  };
  Value::Array(Arc::new(vec![
    Value::Int32(component(1, 2, 2, 1)),
    Value::Int32(component(2, 0, 0, 2)),
    Value::Int32(component(0, 1, 1, 0)),
  ]))
}
