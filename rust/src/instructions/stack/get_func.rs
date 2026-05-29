/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{value::Value, var_stack::VarStack};
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[allow(clippy::ptr_arg)]
#[inline]
pub fn get_func(
  stack: &mut SmallVec<[Value; 16]>,
  vars: &mut VarStack,
  index: usize,
  ip: usize,
) -> Result<(), VMError> {
  let val = unsafe { vars.get_unchecked(index) }.clone();
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  stack.push(val);
  Ok(())
}
