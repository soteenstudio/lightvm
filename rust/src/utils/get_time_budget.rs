/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[cfg(not(feature = "node"))]
use crate::types::time_budget::TimeBudget;
#[cfg(not(feature = "node"))]
pub fn get_time_budget(value: TimeBudget) -> u64 {
  match value {
    TimeBudget::Cheap => 200,
    TimeBudget::Normal => 1000,
    TimeBudget::Expensive => 5000,
  }
}
