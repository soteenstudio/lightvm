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
#[inline(always)]
pub fn powi_f16in(base: f16, exp: i16) -> f16 {
  let mut b = base;
  let mut e = exp;
  let mut res = f16::ONE;
  let is_negative = e < 0;
  if is_negative {
    e = -e;
  }
  while e > 0 {
    if e & 1 == 1 {
      res *= b;
    }
    b *= b;
    e >>= 1;
  }
  if is_negative {
    f16::ONE / res
  } else {
    res
  }
}
