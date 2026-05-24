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
use std::fmt::Write;
#[inline]
pub fn inspect_arr_func(stack: &mut [Value], ip: usize) -> Result<(), VMError> {
  if let Some(top) = stack.last_mut() {
    if let Value::Array(arr) = top {
      let mut result = String::with_capacity(arr.len() * 10 + 2);
      result.push('[');
      for (i, v) in arr.iter().enumerate() {
        if i > 0 {
          result.push_str(", ");
        }
        let _ = write!(result, "{:?}", v);
      }
      result.push(']');
      *top = Value::String(SmolStr::from(result));
      Ok(())
    } else {
      Err(VMError::TypeMismatch {
        ip,
        expected: "Array",
        found: top.type_of(),
      })
    }
  } else {
    Err(VMError::StackUnderflow {
      ip,
      opcode: "INSPECT_ARR",
    })
  }
}
