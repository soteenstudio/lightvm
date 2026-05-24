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
use smol_str::SmolStr;
#[inline(always)]
pub fn push_string_func(stack: &mut Vec<Value>, val: &SmolStr, ip: usize) -> Result<(), VMError> {
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  stack.push(Value::String(val.clone()));
  Ok(())
}
