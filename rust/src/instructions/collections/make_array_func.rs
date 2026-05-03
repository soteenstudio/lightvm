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
pub fn make_array_func(stack: &mut Vec<Value>, count: u32) -> Result<(), SmolStr> {
  let mut arr = Vec::with_capacity(count as usize);
  for _ in 0..count {
    arr.push(
      stack
        .pop()
        .ok_or_else(|| SmolStr::new("Stack underflow: array element"))?,
    );
  }
  arr.reverse();
  stack.push(Value::Array(arr));
  Ok(())
}
