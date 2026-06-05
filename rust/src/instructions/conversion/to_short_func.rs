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
#[cold]
#[inline(never)]
pub fn to_short_values(val: Value) -> Result<Value, &'static str> {
  match val {
    Value::String(s) => s
      .parse::<i16>()
      .map(Value::Int16)
      .map_err(|_| "Unparseable String"),
    ref numeric if numeric.is_number() => Ok(Value::Int16(numeric.as_i16())),
    _ => Err("Unsupported Type"),
  }
}
#[inline(always)]
pub fn to_short_func(stack: &mut [Value], ip: usize) -> Result<(), VMError> {
  if let Some(top) = stack.last_mut() {
    let owned_val = std::mem::take(top);
    *top = to_short_values(owned_val).map_err(|err| VMError::TypeMismatch {
      ip,
      expected: if err == "Unparseable String" {
        "Valid Numeric String"
      } else {
        "Numeric or String"
      },
      found: err,
    })?;
  }
  Ok(())
}
