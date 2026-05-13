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
pub fn ror_i64in(a: i64, b: i64) -> i64 {
  let s = b & 63;
  let right = ((a >> 1) & 0x7FFF_FFFF_FFFF_FFFF) >> (s.wrapping_sub(1) & 63);
  let left = a << (64_i64.wrapping_sub(s) & 63);
  if s == 0 {
    a
  } else {
    right | left
  }
}
