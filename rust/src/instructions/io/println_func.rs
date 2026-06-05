/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::stack::Stack;
use crate::utils::format_output::format_output;
use crate::utils::vmerror::VMError;
#[cold]
#[inline(never)]
pub fn println_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let val_ref = stack.last_mut().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "PRINTLN",
  })?;
  let val = std::mem::take(val_ref);
  format_output(&val, true);
  Ok(())
}
