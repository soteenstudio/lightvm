/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::vmerror::VMError;
use crate::types::instructions::Instructions;
use crate::types::value::FuncMetadata;
use ahash::AHashMap;
#[cold]
pub fn validate_bytecode(
  bytecode: &[Instructions],
  functions: &AHashMap<SmolStr, FuncMetadata>,
) -> Result<(), VMError> {
  let len = bytecode.len();
  for (ip, instr) in bytecode.iter().enumerate() {
    match instr {
      Instructions::IfFalse(target) | Instructions::Jump(target) | Instructions::Break(target)
        if *target >= len =>
      {
        return Err(VMError::OutOfBounds {
          ip,
          index: *target,
          len,
        });
      }
      _ => {}
    }
  }
  for meta in functions.values() {
    if meta.start >= len {
      return Err(VMError::OutOfBounds {
        ip: meta.start,
        index: meta.start,
        len,
      });
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_valid_bytecode() {
    let bytecode = vec![Instructions::Jump(0)];
    assert!(validate_bytecode(&bytecode, &AHashMap::new()).is_ok());
  }

  #[test]
  fn rejects_invalid_jump_with_structured_error() {
    let result = validate_bytecode(&[Instructions::Jump(1)], &AHashMap::new());
    assert!(matches!(
      result,
      Err(VMError::OutOfBounds {
        ip: 0,
        index: 1,
        len: 1
      })
    ));
  }
}
