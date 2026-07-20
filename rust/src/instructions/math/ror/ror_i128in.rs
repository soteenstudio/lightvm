/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[inline(always)]
pub fn ror_i128in(a: i128, b: i128) -> i128 {
  let s = b & 127;
  if s == 0 {
    return a;
  }
  let right = ((a >> 1) & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) >> (s - 1);
  let left = a << (128 - s);
  right | left
}
