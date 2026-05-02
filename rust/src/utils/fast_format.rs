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
use std::borrow::Cow;
pub fn int_to_cow<T: Into<i64>>(n: T) -> Cow<'static, str> {
  let mut buffer = itoa::Buffer::new();
  Cow::Owned(buffer.format(n.into()).to_owned())
}
pub fn float_to_cow<T: Into<f64>>(f: T) -> Cow<'static, str> {
  let mut buffer = ryu::Buffer::new();
  Cow::Owned(buffer.format(f.into()).to_owned())
}
