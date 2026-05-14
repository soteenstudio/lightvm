/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use half::f16;
use num_traits::Float;
#[inline(always)]
pub fn tan_f16in(a: f16) -> f16 {
  let res = a.tan();
  if res.is_infinite() || res.is_nan() {
    return f16::NAN;
  }
  res
}
