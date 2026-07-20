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
pub fn dup_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let top = stack
    .last()
    .ok_or(VMError::StackUnderflow { ip, opcode: "DUP" })?;
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  let cloned = top.clone();
  stack.push(cloned);
  Ok(())
}
