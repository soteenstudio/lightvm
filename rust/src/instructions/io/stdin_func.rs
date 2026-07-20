/*
 * Copyright 2025-2026 SoTeen Studio
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
use smol_str::SmolStr;
use std::io::{self, Write};
#[cold]
#[inline(never)]
pub fn stdin_func(stack: &mut Stack) -> Result<(), VMError> {
  let _ = io::stdout().flush();
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .map_err(|e| VMError::SystemError(SmolStr::new(format!("Failed to read stdin: {}", e))))?;
  let trimmed = input.trim_end_matches(['\r', '\n']);
  stack.push(Value::String(SmolStr::new(trimmed)));
  Ok(())
}
