/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::vmerror::VMError;
use crate::types::security_config::SecurityConfig;
pub struct GasMonitor {
  pub max_ticks: u64,
}
impl GasMonitor {
  pub fn new(config: &SecurityConfig) -> Result<Self, VMError> {
    if config.max_ticks == 0 {
      return Err(VMError::InvalidMaxTicksConfig);
    }
    Ok(Self {
      max_ticks: config.max_ticks,
    })
  }
  #[inline(always)]
  pub fn check_tick(&self, tick: u64) -> Result<(), VMError> {
    if tick >= self.max_ticks {
      return Err(VMError::TickLimitExceeded);
    }
    Ok(())
  }
}
