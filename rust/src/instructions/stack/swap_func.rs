/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::stack::Stack;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn swap_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let len = stack.len();
  if len < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "SWAP" });
  }
  stack.swap(len - 1, len - 2);
  Ok(())
}
