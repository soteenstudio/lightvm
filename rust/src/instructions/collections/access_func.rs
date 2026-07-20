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
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use smol_str::SmolStr;
#[inline(always)]
pub fn access_func(stack: &mut Stack, prop: &SmolStr, ip: usize) -> Result<(), VMError> {
  if let Some(top) = stack.last_mut() {
    if let Value::Object(map) = top {
      let result = map.get(prop).cloned().unwrap_or(Value::Undefined);
      *top = result;
      Ok(())
    } else {
      Err(VMError::TypeMismatch {
        ip,
        expected: "Object",
        found: "Non-Object",
      })
    }
  } else {
    Err(VMError::StackUnderflow {
      ip,
      opcode: "ACCESS",
    })
  }
}
