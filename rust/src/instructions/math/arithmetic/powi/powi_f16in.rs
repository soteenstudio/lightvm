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
#[inline(always)]
pub fn powi_f16in(base: f16, exp: i16) -> f16 {
  let mut b = base;
  let mut e = exp.unsigned_abs();
  let mut res = f16::ONE;
  while e > 0 {
    if e & 1 == 1 {
      res *= b;
    }
    b *= b;
    e >>= 1;
  }
  let final_res = if exp < 0 { f16::ONE / res } else { res };
  if final_res.is_infinite() || final_res.is_nan() {
    return f16::NAN;
  }
  final_res
}
