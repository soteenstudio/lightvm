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
pub fn powi_f64in(base: f64, exp: i64) -> f64 {
  let mut b = base;
  let mut e = exp;
  let mut res = 1.0f64;
  let is_negative = e < 0;
  if is_negative {
    e = e.wrapping_abs();
  }
  while e > 0 {
    if e & 1 == 1 {
      res *= b;
    }
    b *= b;
    e >>= 1;
  }
  if is_negative {
    1.0 / res
  } else {
    res
  }
}
