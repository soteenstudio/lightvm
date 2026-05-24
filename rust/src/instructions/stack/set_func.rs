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
#[inline]
pub fn set_func(
  stack: &mut Vec<Value>,
  vars: &mut Vec<Value>,
  index: usize,
  ip: usize,
) -> Result<(), VMError> {
  let val = stack
    .pop()
    .ok_or(VMError::StackUnderflow { ip, opcode: "SET" })?;
  if index >= vars.len() {
    vars.resize(index + 1, Value::Undefined);
  }
  vars[index] = val;
  Ok(())
}
