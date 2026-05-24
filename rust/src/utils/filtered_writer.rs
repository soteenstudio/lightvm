/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use std::fmt::{self};
/// Struct pembantu buat nge-filter teks pas lagi ditulis
pub struct FilteredWriter {
  pub buffer: String,
  pub state: usize,
}
impl fmt::Write for FilteredWriter {
  #[inline]
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for &b in s.as_bytes() {
      let c = b as char;
      match (self.state, c) {
        (_, '\'' | '\"') => continue,
        (0, ':') => self.state = 1,
        (1, ':') => self.state = 2,
        (2, 's') => self.state = 3,
        (3, 'p') => self.state = 4,
        (4, 'a') => self.state = 5,
        (5, 'c') => self.state = 6,
        (6, 'e') => {
          self.buffer.push(' ');
          self.state = 0;
        }
        (2, 't') => self.state = 10,
        (10, 'r') => self.state = 11,
        (11, 'i') => self.state = 12,
        (12, 'n') => self.state = 13,
        (13, 'g') => {
          self.state = 0;
        }
        (s_val, char_input) => {
          if s_val > 0 {
            self.flush_failed_match();
          }
          self.buffer.push(char_input);
          self.state = 0;
        }
      }
    }
    Ok(())
  }
}
impl FilteredWriter {
  #[inline(always)]
  pub fn flush_failed_match(&mut self) {
    match self.state {
      1 => self.buffer.push(':'),
      2 => self.buffer.push_str("::"),
      3 => self.buffer.push_str("::s"),
      4 => self.buffer.push_str("::sp"),
      5 => self.buffer.push_str("::spa"),
      6 => self.buffer.push_str("::spac"),
      10 => self.buffer.push_str("::t"),
      11 => self.buffer.push_str("::tr"),
      12 => self.buffer.push_str("::tri"),
      13 => self.buffer.push_str("::trin"),
      _ => {}
    }
    self.state = 0;
  }
}
