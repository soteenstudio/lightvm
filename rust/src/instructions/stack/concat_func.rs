/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use crate::utils::filtered_writer::FilteredWriter;
use std::fmt::Write;
#[inline]
pub fn concat_func(a: &Value, b: &Value) -> Value {
  let mut f_writer = FilteredWriter {
    buffer: String::with_capacity(64),
    pending: String::with_capacity(8),
  };
  let _ = write!(f_writer, "{}{}", a, b);
  if !f_writer.pending.is_empty() {
    f_writer.buffer.push_str(&f_writer.pending);
  }
  Value::String(f_writer.buffer)
}
