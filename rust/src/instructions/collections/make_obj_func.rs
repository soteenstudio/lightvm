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
use ahash::AHashMap;
use std::sync::Arc;
const MAX_OBJECT_CAPACITY: usize = 10_000;
#[cold]
#[inline(never)]
pub fn make_obj_func(stack: &mut Stack, count: u32, ip: usize) -> Result<(), VMError> {
  let count_usize = count as usize;
  if count_usize > MAX_OBJECT_CAPACITY {
    return Err(VMError::StackOverflow {
      ip,
      limit: MAX_OBJECT_CAPACITY,
    });
  }
  let mut obj = AHashMap::with_capacity(count_usize);
  for _ in 0..count {
    let val = stack.pop().ok_or(VMError::StackUnderflow {
      ip,
      opcode: "MAKE_OBJ (value)",
    })?;
    let key_raw = stack.pop().ok_or(VMError::StackUnderflow {
      ip,
      opcode: "MAKE_OBJ (key)",
    })?;
    if let Value::String(s) = key_raw {
      obj.insert(s, val);
    } else {
      return Err(VMError::TypeMismatch {
        ip,
        expected: "String (Key)",
        found: key_raw.type_of(),
      });
    }
  }
  stack.push(Value::Object(Arc::new(obj)));
  Ok(())
}
