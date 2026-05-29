/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use crate::types::value::FuncMetadata;
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use smol_str::SmolStr;
pub fn validate_bytecode(
  bytecode: &[Instructions],
  functions: &AHashMap<SmolStr, FuncMetadata>,
) -> Result<(), VMError> {
  let len = bytecode.len();
  for (ip, instr) in bytecode.iter().enumerate() {
    match instr {
      Instructions::Jump(target) | Instructions::IfFalse(target) | Instructions::Break(target) => {
        if *target >= len {
          return Err(VMError::OutOfBounds {
            ip,
            index: *target,
            len,
          });
        }
      }
      _ => {}
    }
  }
  for (name, meta) in functions {
    if meta.start >= len {
      return Err(VMError::SystemError(SmolStr::from(format!(
        "Function '{}' start address {} is out of bounds (len: {})",
        name, meta.start, len
      ))));
    }
  }
  Ok(())
}
