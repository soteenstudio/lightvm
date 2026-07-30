/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use crate::types::security_config::SecurityConfig;
use crate::utils::vmerror::VMError;
use smol_str::SmolStr;
pub fn validate_security(
  bytecode: &[Instructions],
  config: &SecurityConfig,
) -> Result<(), VMError> {
  if config.unsafe_mode {
    return Ok(());
  }
  let mut io_count = 0;
  let mut import_count = 0;
  let mut alloc_count = 0;
  let mut call_count = 0;
  let mut jump_count = 0;
  for (ip, instr) in bytecode.iter().enumerate() {
    match instr {
      Instructions::Print
      | Instructions::Println
      | Instructions::Stdout
      | Instructions::Stdoutln => {
        io_count += 1;
        if io_count > config.max_io {
          return Err(VMError::SystemError(SmolStr::from(format!(
            "Security Violation: I/O Flood at IP {}",
            ip
          ))));
        }
      }
      Instructions::Import(module, _) => {
        import_count += 1;
        if import_count > config.max_import {
          return Err(VMError::SystemError(SmolStr::from(
            "Security Violation: Too many imports",
          )));
        }
        if !config.allowed_imports.contains(&module.to_string()) {
          return Err(VMError::SystemError(SmolStr::from(format!(
            "Security Violation: Forbidden module '{}'",
            module
          ))));
        }
      }
      Instructions::MakeObj(_) | Instructions::MakeArray(_) => {
        alloc_count += 1;
        if alloc_count > config.max_alloc {
          return Err(VMError::SystemError(SmolStr::from(
            "Security Violation: Memory limit reached",
          )));
        }
      }
      Instructions::Call(_, _) => {
        call_count += 1;
        if call_count > config.max_call {
          return Err(VMError::SystemError(SmolStr::from(
            "Security Violation: Excessive calls",
          )));
        }
      }
      Instructions::Jump(_) | Instructions::IfFalse(_) | Instructions::Break(_) => {
        jump_count += 1;
        if jump_count > config.max_jump {
          return Err(VMError::SystemError(SmolStr::from(
            "Security Violation: Excessive jumps",
          )));
        }
      }
      _ => {}
    }
  }
  Ok(())
}
