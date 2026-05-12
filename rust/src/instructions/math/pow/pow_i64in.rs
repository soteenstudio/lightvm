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
pub fn pow_i64in(a: i64, b: i64) -> i64 {
  a.wrapping_pow(b as u32)
}
