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
#[inline]
pub fn set_prop_func(stack: &mut Vec<Value>, prop: &SmolStr, ip: usize) -> Result<(), VMError> {
  let val = stack.pop().ok_or_else(|| VMError::StackUnderflow {
    ip,
    opcode: "SET_PROP (value)",
  })?;
  if let Some(top) = stack.last_mut() {
    if let Value::Object(ref mut map_arc) = top {
      let map = Arc::make_mut(map_arc);
      map.insert(prop.clone(), val);
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
      opcode: "SET_PROP (object)",
    })
  }
}
