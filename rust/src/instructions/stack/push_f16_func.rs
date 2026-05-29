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
use half::f16;
use smallvec::SmallVec;
#[inline(always)]
pub fn push_f16_func(
  stack: &mut SmallVec<[Value; 16]>,
  val: &f16,
  ip: usize,
) -> Result<(), VMError> {
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  stack.push(Value::Float16(*val));
  Ok(())
}
