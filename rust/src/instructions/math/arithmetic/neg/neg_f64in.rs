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
pub fn neg_f64in(a: f64) -> f64 {
  let res = -a;
  if res.is_nan() {
    return f64::NAN;
  }
  res
}
