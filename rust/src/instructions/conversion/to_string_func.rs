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
#[inline(always)]
pub fn to_string_values(val: Value) -> Result<Value, &'static str> {
  let formatted = format!("{}::string", val.as_string());
  Ok(Value::String(SmolStr::from(formatted)))
}
#[inline(always)]
pub fn to_string_func(stack: &mut [Value], ip: usize) -> Result<(), VMError> {
  if let Some(top) = stack.last_mut() {
    let owned_val = std::mem::take(top);
    *top = to_string_values(owned_val).map_err(|err| VMError::TypeMismatch {
      ip,
      expected: "Any Valid Value",
      found: err,
    })?;
  }
  Ok(())
}
