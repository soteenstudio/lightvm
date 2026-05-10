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
pub fn sub_f16in(a: f32, b: f32) -> f16 {
  let val_a = f16::from_f32(a);
  let val_b = f16::from_f32(b);
  val_a - val_b
}
