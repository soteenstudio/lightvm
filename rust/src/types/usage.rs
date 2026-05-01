/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Usage<'a> {
  #[serde(borrow)]
  pub read: HashSet<&'a str>,
  #[serde(borrow)]
  pub written: HashSet<&'a str>,
}
impl<'a> Usage<'a> {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn add_read(&mut self, variable: &'a str) {
    self.read.insert(variable);
  }
  pub fn add_written(&mut self, variable: &'a str) {
    self.written.insert(variable);
  }
}
