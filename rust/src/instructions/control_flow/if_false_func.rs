/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn if_false_func(stack: &mut SmallVec<[Value; 16]>, ip: usize) -> Result<bool, VMError> {
  let cond_ref = stack.last_mut().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "IF_FALSE",
  })?;
  let cond = std::mem::take(cond_ref);
  Ok(!cond.is_truthy())
}
