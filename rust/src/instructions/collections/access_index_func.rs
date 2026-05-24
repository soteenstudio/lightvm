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
#[inline(always)]
pub fn access_index_func(stack: &mut Vec<Value>, ip: usize) -> Result<(), VMError> {
  let index_val = stack.pop().ok_or_else(|| VMError::StackUnderflow {
    ip,
    opcode: "ACCESS_INDEX (index)",
  })?;
  if let Some(top) = stack.last_mut() {
    match (&mut *top, index_val) {
      (Value::Array(arr), Value::Int64(idx)) => {
        let i = idx as usize;
        if i < arr.len() {
          *top = arr[i].clone();
          Ok(())
        } else {
          Err(VMError::OutOfBounds {
            ip,
            index: i,
            len: arr.len(),
          })
        }
      }
      (Value::Array(_), _) => Err(VMError::TypeMismatch {
        ip,
        expected: "Int64 (Index)",
        found: "Invalid Index Type",
      }),
      _ => Err(VMError::TypeMismatch {
        ip,
        expected: "Array",
        found: "Non-Array",
      }),
    }
  } else {
    Err(VMError::StackUnderflow {
      ip,
      opcode: "ACCESS_INDEX (array)",
    })
  }
}
