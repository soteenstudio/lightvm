/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

#[inline(always)]
pub fn pow_i32in(base: i32, exp: i32) -> i32 {
  let mut b = base;
  let mut e = exp;
  let mut res = 1i32;
  if e < 0 {
    return 0;
  }
  while e > 0 {
    if e & 1 == 1 {
      res = res.wrapping_mul(b);
    }
    b = b.wrapping_mul(b);
    e >>= 1;
  }
  res
}
