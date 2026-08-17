/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use half::f16;
use num_traits::Float;
#[inline(always)]
pub fn log2_f16in(a: f16) -> f16 {
  if a <= f16::from_f32(0.0) {
    return f16::NAN;
  }
  let res = a.log2();
  if res.is_nan() {
    return f16::NAN;
  }
  res
}
