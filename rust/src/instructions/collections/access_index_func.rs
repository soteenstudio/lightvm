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
#[inline(always)]
pub fn access_index_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let index_val = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "ACCESS_INDEX (index)",
  })?;
  if let Some(top) = stack.last_mut() {
    match &mut *top {
      Value::Array(arr) => {
        if !index_val.is_number() {
          return Err(VMError::TypeMismatch {
            ip,
            expected: "Number (Index)",
            found: "Invalid Index Type",
          });
        }
        let idx = index_val.as_i64();
        if idx < 0 {
          return Err(VMError::OutOfBounds {
            ip,
            index: 0,
            len: arr.len(),
          });
        }
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
