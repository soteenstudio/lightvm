/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use std::fmt::{self};
/// Struct pembantu buat nge-filter teks pas lagi ditulis
pub struct FilteredWriter {
  pub buffer: String,
  pub pending: String,
}
impl fmt::Write for FilteredWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      match c {
        '\'' | '\"' => continue,
        ':' => {
          self.pending.push(':');
          if self.pending == "::string" {
            self.pending.clear();
          } else if self.pending == "::space" {
            self.buffer.push(' ');
            self.pending.clear();
          }
        }
        _ => {
          if !self.pending.is_empty() {
            self.buffer.push_str(&self.pending);
            self.pending.clear();
          }
          self.buffer.push(c);
        }
      }
    }
    Ok(())
  }
}
