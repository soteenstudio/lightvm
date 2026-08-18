/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::time_budget::TimeBudget;
use serde::{Deserialize, Serialize};
fn default_time_budget() -> TimeBudget {
  TimeBudget::Cheap
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SecurityConfig {
  pub max_io: usize,
  pub max_import: usize,
  pub max_alloc: usize,
  pub max_call: usize,
  pub max_jump: usize,
  pub max_ticks: u64,
  pub max_stack_size: usize,
  pub allowed_imports: Vec<String>,
  pub unsafe_mode: bool,
  #[serde(default = "default_time_budget")]
  pub time_budget: TimeBudget,
}
impl Default for SecurityConfig {
  fn default() -> Self {
    Self {
      max_io: 100,
      max_import: 3,
      max_alloc: 50,
      max_call: 200,
      max_jump: 100,
      max_ticks: 1_000_000,
      max_stack_size: 128,
      allowed_imports: vec!["math".into(), "time".into(), "utils".into()],
      unsafe_mode: false,
      time_budget: TimeBudget::Cheap,
    }
  }
}
