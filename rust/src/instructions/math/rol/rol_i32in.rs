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
pub fn rol_i32in(a: i32, b: i32) -> i32 {
  let s = b & 31;
  if s == 0 {
    return a;
  }
  let left = a << s;
  let right = ((a >> 1) & 0x7FFFFFFF) >> (31 - s);
  left | right
}
