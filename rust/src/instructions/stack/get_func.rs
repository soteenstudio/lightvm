/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[allow(clippy::ptr_arg)]
#[inline]
pub fn get_func(
  stack: &mut Vec<Value>,
  vars: &mut Vec<Value>,
  index: usize,
  ip: usize,
) -> Result<(), VMError> {
  let val = vars.get(index).cloned().ok_or(VMError::OutOfBounds {
    ip,
    index,
    len: vars.len(),
  })?;
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  stack.push(val);
  Ok(())
}
