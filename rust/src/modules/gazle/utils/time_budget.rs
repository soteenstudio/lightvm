/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::time::{Duration, Instant};
pub struct TimeBudget {
  start: Instant,
  limit: Duration,
}
impl TimeBudget {
  pub fn new(millis: u64) -> Self {
    Self {
      start: Instant::now(),
      limit: Duration::from_millis(millis),
    }
  }
  pub fn is_expired(&self) -> bool {
    self.start.elapsed() > self.limit
  }
}
