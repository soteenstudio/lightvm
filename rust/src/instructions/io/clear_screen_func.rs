/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::vmerror::VMError;
use std::io;
use std::io::Write;
#[inline(always)]
pub fn clear_screen_func() -> Result<(), VMError> {
  let mut out = io::stdout().lock();
  out.write_all(b"\x1B[2J\x1B[1H").map_err(|e| {
    VMError::SystemError(smol_str::SmolStr::new(format!(
      "Clear screen failed: {}",
      e
    )))
  })?;
  out.flush().map_err(|e| {
    VMError::SystemError(smol_str::SmolStr::new(format!(
      "Clear screen flush failed: {}",
      e
    )))
  })?;
  Ok(())
}
