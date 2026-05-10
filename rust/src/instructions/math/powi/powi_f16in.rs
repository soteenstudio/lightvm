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
pub fn powi_f16in(a: f16, b: i16) -> f16 {
  // 1. Konversi half (f16) ke f32 karena f16 native gak punya method .powi()
  // 2. b di-cast ke i32 karena standar method .powi() di Rust minta i32
  let val_f32 = a.to_f32();
  let res_f32 = val_f32.powi(b as i32);
  
  // 3. Balikin lagi ke f16
  f16::from_f32(res_f32)
}
