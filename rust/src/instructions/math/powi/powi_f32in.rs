/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

#[inline(always)]
pub fn powi_f32in(base: f32, exp: i32) -> f32 {
  let mut b = base;
  let mut e = exp.unsigned_abs();
  let mut res = 1.0f32;
  while e > 0 {
    if e & 1 == 1 {
      res *= b;
    }
    b *= b;
    e >>= 1;
  }
  let final_res = if exp < 0 { 1.0 / res } else { res };
  if final_res.is_infinite() || final_res.is_nan() {
    return f32::NAN;
  }
  final_res
}
