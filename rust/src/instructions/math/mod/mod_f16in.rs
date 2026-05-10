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
pub fn mod_f16in(a: u16, b: u16) -> u16 {
  let val_a = f16::from_bits(a);
  let val_b = f16::from_bits(b);
  let result = val_a % val_b;
  result.to_bits()
}
