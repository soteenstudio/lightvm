/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use smol_str::SmolStr;
use std::sync::Arc;
#[inline(always)]
pub fn shrink_func(stack: &mut Vec<Value>, ip: usize) -> Result<(), VMError> {
  let len_val = stack.pop().ok_or_else(|| VMError::StackUnderflow {
    ip,
    opcode: "SHRINK (length)",
  })?;
  let length = match len_val {
    Value::Int16(i) => i as usize,
    Value::Int32(i) => i as usize,
    Value::Int64(i) => i as usize,
    Value::Int128(i) => i as usize,
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: "Integer (Length)",
        found: "Non-Integer Length",
      })
    }
  };
  if let Some(target) = stack.last_mut() {
    match target {
      Value::String(s) => {
        let mut s_val = s.to_string();
        s_val.truncate(length);
        *target = Value::String(SmolStr::from(s_val));
        Ok(())
      }
      Value::Array(arr) => {
        let mut a_val = (**arr).clone();
        a_val.truncate(length);
        *target = Value::Array(Arc::new(a_val));
        Ok(())
      }
      _ => Err(VMError::TypeMismatch {
        ip,
        expected: "String or Array",
        found: "Invalid Shrink Target",
      }),
    }
  } else {
    Err(VMError::StackUnderflow {
      ip,
      opcode: "SHRINK (target)",
    })
  }
}
