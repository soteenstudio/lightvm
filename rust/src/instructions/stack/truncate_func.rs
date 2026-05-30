/*
 * Copyright 2026 SoTeen Studio
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
#[inline(always)]
pub fn truncate_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let val = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "TRUNCATE",
  })?;
  let target_size = match val {
    Value::Int16(i) => i as usize,
    Value::Int32(i) => i as usize,
    Value::Int64(i) => i as usize,
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: "Integer",
        found: "Unknown",
      });
    }
  };
  if target_size <= stack.len() {
    stack.truncate(target_size);
    Ok(())
  } else {
    Err(VMError::OutOfBounds {
      ip,
      index: target_size,
      len: stack.len(),
    })
  }
}
