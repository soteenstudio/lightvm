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
#[inline(always)]
pub fn jump_threading(bytecode: &mut [Instructions]) {
  let mut changed = true;
  while changed {
    changed = false;
    for i in 0..bytecode.len() {
      if let Instructions::Jump(target) = bytecode[i] {
        if let Some(final_target) = find_final_target(bytecode, target)
          && final_target != target
        {
          bytecode[i] = Instructions::Jump(final_target);
          changed = true;
        }
      } else if let Instructions::IfFalse(target) = bytecode[i]
        && let Some(final_target) = find_final_target(bytecode, target)
        && final_target != target
      {
        bytecode[i] = Instructions::IfFalse(final_target);
        changed = true;
      }
    }
  }
}
fn find_final_target(bytecode: &[Instructions], mut target: usize) -> Option<usize> {
  let mut visited = std::collections::HashSet::new();
  while let Some(Instructions::Jump(next_target)) = bytecode.get(target) {
    if !visited.insert(target) {
      return None;
    }
    target = *next_target;
  }
  Some(target)
}
