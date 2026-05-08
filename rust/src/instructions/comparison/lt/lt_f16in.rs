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
pub fn lt_f16in(a: u16, b: u16) -> bool {
  let val_a = f16::from_bits(a);
  let val_b = f16::from_bits(b);
  val_a < val_b
}
