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
pub fn powf_f16in(a: f16, b: f16) -> f16 {
  let val_a = a.to_f32();
  let val_b = b.to_f32();
  let res = val_a.powf(val_b);
  f16::from_f32(res)
}
