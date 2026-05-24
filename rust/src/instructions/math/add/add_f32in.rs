/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[inline(always)]
pub fn add_f32in(a: f32, b: f32) -> f32 {
  let res = a + b;
  if res.is_infinite() || res.is_nan() {
    return f32::NAN;
  }
  res
}
