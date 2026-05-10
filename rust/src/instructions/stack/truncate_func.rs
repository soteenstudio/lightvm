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
#[inline(always)]
pub fn truncate_func(stack: &mut Vec<Value>) -> Result<(), SmolStr> {
  let val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on TRUNCATE"))?;
  let target_size = match val {
    Value::Int16(i) => i as usize,
    Value::Int32(i) => i as usize,
    Value::Int64(i) => i as usize,
    _ => return Err(SmolStr::new("TRUNCATE target size must be an integer")),
  };
  if target_size <= stack.len() {
    stack.truncate(target_size);
  } else {
    return Err(SmolStr::new(
      "TRUNCATE target size exceeds current stack height",
    ));
  }
  Ok(())
}
