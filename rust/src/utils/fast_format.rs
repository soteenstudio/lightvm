/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use smol_str::SmolStr;
pub trait ToRyuFloat {
  fn format_to_smol(self) -> SmolStr;
}
impl ToRyuFloat for f32 {
  #[inline]
  fn format_to_smol(self) -> SmolStr {
    let mut buffer = ryu::Buffer::new();
    SmolStr::new(buffer.format(self))
  }
}
impl ToRyuFloat for f64 {
  #[inline]
  fn format_to_smol(self) -> SmolStr {
    let mut buffer = ryu::Buffer::new();
    SmolStr::new(buffer.format(self))
  }
}
impl ToRyuFloat for half::f16 {
  #[inline]
  fn format_to_smol(self) -> SmolStr {
    let mut buffer = ryu::Buffer::new();
    SmolStr::new(buffer.format(self.to_f32()))
  }
}
#[inline]
pub fn int_to_smol<T: itoa::Integer>(n: T) -> SmolStr {
  let mut buffer = itoa::Buffer::new();
  SmolStr::new(buffer.format(n))
}
#[inline]
pub fn float_to_smol<T: ToRyuFloat>(f: T) -> SmolStr {
  f.format_to_smol()
}
