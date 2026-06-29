/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::cell::RefCell;
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

thread_local! {
  static THREAD_ERROR_CONFIG: RefCell<Option<VMErrorContainer>> = RefCell::new(None);
}

static EXPLAIN_MODE: OnceLock<Mutex<VMErrorContainer>> = OnceLock::new();

pub fn set_thread_error_config(backtrace: bool, explain: bool, hint: bool) {
  THREAD_ERROR_CONFIG.with(|config| {
    *config.borrow_mut() = Some(VMErrorContainer {
      backtrace,
      explain,
      hint,
    });
  });
}

pub fn get_error_config() -> &'static Mutex<VMErrorContainer> {
  EXPLAIN_MODE.get_or_init(|| Mutex::new(VMErrorContainer::new()))
}

pub fn get_thread_or_global_config() -> VMErrorContainer {
  THREAD_ERROR_CONFIG.with(|config| {
    if let Some(cfg) = config.borrow().as_ref() {
      cfg.get_value()
    } else {
      match get_error_config().lock() {
        Ok(guard) => guard.get_value(),
        Err(poisoned) => poisoned.into_inner().get_value(),
      }
    }
  })
}
