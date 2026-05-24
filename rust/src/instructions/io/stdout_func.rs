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
use std::io::{self, Write};
#[inline(always)]
pub fn stdout_func(stack: &mut Vec<Value>, ip: usize) -> Result<(), VMError> {
  let val = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "STDOUT",
  })?;
  if let Value::String(s) = val {
    let mut out = io::stdout().lock();
    out.write_all(s.as_bytes()).map_err(|e| {
      VMError::SystemError(smol_str::SmolStr::new(format!(
        "Stdout write failed: {}",
        e
      )))
    })?;
    out.flush().map_err(|e| {
      VMError::SystemError(smol_str::SmolStr::new(format!(
        "Stdout flush failed: {}",
        e
      )))
    })?;
    Ok(())
  } else {
    Err(VMError::TypeMismatch {
      ip,
      expected: "String",
      found: "Non-String Value",
    })
  }
}
#[inline(always)]
pub fn stdoutln_func(stack: &mut Vec<Value>, ip: usize) -> Result<(), VMError> {
  let val = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "STDOUTLN",
  })?;
  if let Value::String(s) = val {
    let mut out = io::stdout().lock();
    out.write_all(s.as_bytes()).map_err(|e| {
      VMError::SystemError(smol_str::SmolStr::new(format!(
        "Stdoutln write failed: {}",
        e
      )))
    })?;
    out.write_all(b"\n").map_err(|e| {
      VMError::SystemError(smol_str::SmolStr::new(format!(
        "Stdoutln newline failed: {}",
        e
      )))
    })?;
    Ok(())
  } else {
    Err(VMError::TypeMismatch {
      ip,
      expected: "String",
      found: "Non-String Value",
    })
  }
}
