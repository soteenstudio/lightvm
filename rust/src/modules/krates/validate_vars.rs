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
use crate::utils::vmerror::VMError;
pub fn validate_vars(bytecode: &[Instructions], var_count: usize) -> Result<(), VMError> {
  for (ip, instr) in bytecode.iter().enumerate() {
    let idx = match instr {
      Instructions::ValIdx(i)
      | Instructions::SetIdx(i)
      | Instructions::GetIdx(i)
      | Instructions::IncIdx(i, _)
      | Instructions::DecIdx(i, _) => Some(*i),
      _ => None,
    };
    if let Some(i) = idx
      && i >= var_count
    {
      return Err(VMError::OutOfBounds {
        ip,
        index: i,
        len: var_count,
      });
    }
  }
  Ok(())
}
