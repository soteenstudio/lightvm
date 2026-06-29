/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::sync::Mutex;
use std::sync::OnceLock;
pub struct VMErrorContainer {
  pub backtrace: bool,
  pub explain: bool,
  pub hint: bool,
}
impl Default for VMErrorContainer {
  fn default() -> Self {
    Self::new()
  }
}
impl VMErrorContainer {
  pub fn new() -> Self {
    Self {
      backtrace: false,
      explain: false,
      hint: true,
    }
  }
  pub fn get_value(&self) -> VMErrorContainer {
    VMErrorContainer {
      backtrace: self.backtrace,
      explain: self.explain,
      hint: self.hint,
    }
  }
  pub(crate) fn set_value(&mut self, backtrace: bool, explain: bool, hint: bool) {
    self.backtrace = backtrace;
    self.explain = explain;
    self.hint = hint;
  }
}
static EXPLAIN_MODE: OnceLock<Mutex<VMErrorContainer>> = OnceLock::new();
pub fn get_error_config() -> &'static Mutex<VMErrorContainer> {
  EXPLAIN_MODE.get_or_init(|| Mutex::new(VMErrorContainer::new()))
}
