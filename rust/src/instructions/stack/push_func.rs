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
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::get_type_name::get_type_name;
#[inline]
pub fn push_func(stack: &mut Stack, val: Value, ip: usize) -> Result<(), VMError> {
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  if val.is_array() || val.is_object() {
    return Err(VMError::InvalidValue {
      ip,
      value: get_type_name(val),
    });
  }
  stack.push(val);
  Ok(())
}
