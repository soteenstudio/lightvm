/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use itoa;
use ryu;
use smol_str::SmolStr;
#[inline]
pub fn int_to_smol<T: Into<i64>>(n: T) -> SmolStr {
  let mut buffer = itoa::Buffer::new();
  SmolStr::new(buffer.format(n.into()))
}
#[inline]
pub fn float_to_smol<T: Into<f64>>(f: T) -> SmolStr {
  let mut buffer = ryu::Buffer::new();
  SmolStr::new(buffer.format(f.into()))
}
