/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use smol_str::SmolStr;
pub fn access_index_func(stack: &mut Vec<Value>) -> Result<(), SmolStr> {
  let index_val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow: missing index"))?;
  if let Some(top) = stack.last_mut() {
    match (&mut *top, index_val) {
      (Value::Array(arr), Value::Int64(idx)) => {
        let i = idx as usize;
        if i < arr.len() {
          *top = arr[i].clone();
          Ok(())
        } else {
          Err(SmolStr::new("Array index out of bounds"))
        }
      }
      (Value::Array(_), _) => Err(SmolStr::new("Array index must be an integer")),
      _ => Err(SmolStr::new("Cannot access index of non-array")),
    }
  } else {
    Err(SmolStr::new("Stack underflow: missing array object"))
  }
}
