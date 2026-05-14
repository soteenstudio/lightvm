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
pub fn rol_i16in(a: i16, b: i16) -> i16 {
  let s = b & 15;
  if s == 0 {
    return a;
  }
  let left = a << s;
  let right = ((a >> 1) & 0x7FFF) >> (15 - s);
  left | right
}
