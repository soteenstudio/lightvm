/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::security_config::SecurityConfig;
use smol_str::SmolStr;
pub struct GasMonitor {
  pub max_ticks: u64,
}
impl GasMonitor {
  pub fn new(config: &SecurityConfig) -> Result<Self, SmolStr> {
    if config.max_ticks == 0 {
      return Err(SmolStr::from(
        "Security Config Error: max_ticks cannot be 0. Please set a valid limit or remove the restriction.",
      ));
    }
    Ok(Self {
      max_ticks: config.max_ticks,
    })
  }
  #[inline(always)]
  pub fn check_tick(&self, tick: u64) -> Result<(), SmolStr> {
    if tick > self.max_ticks {
      return Err(SmolStr::from(
        "Security Violation: Gas limit exceeded (Infinite loop detected)",
      ));
    }
    Ok(())
  }
}
