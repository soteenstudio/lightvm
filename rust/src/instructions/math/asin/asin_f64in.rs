/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[inline(always)]
pub fn asin_f64in(a: f64) -> f64 {
  if !(-1.0..=1.0).contains(&a) {
    return f64::NAN;
  }
  let res = a.asin();
  if res.is_nan() {
    return f64::NAN;
  }
  res
}
