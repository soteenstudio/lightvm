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
pub fn length_values(val: Value) -> Result<Value, &'static str> {
  let len = match val {
    Value::String(s) => s.len(),
    Value::Array(a) => a.len(),
    Value::Object(obj) => obj.len(),
    _ => return Err("Type does not support length property"),
  };
  Ok(Value::Int64(len as i64))
}
#[inline(always)]
pub fn length_func(stack: &mut [Value], ip: usize) -> Result<(), VMError> {
  if let Some(top) = stack.last_mut() {
    let owned_val = std::mem::take(top);
    *top = length_values(owned_val).map_err(|err| VMError::TypeMismatch {
      ip,
      expected: "String, Array, or Object",
      found: err,
    })?;
  }
  Ok(())
}
