/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::optimizer::{
  analyze_usage::analyze_usage, eliminate_dead_loops::eliminate_dead_loops,
  eliminate_dead_stores::eliminate_dead_stores, fold_constants::fold_constants,
  fold_conversions::fold_conversions, jump_threading::jump_threading,
};
use crate::types::instructions::Instructions;
use std::borrow::Cow;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  fold_constants(&mut bytecode);
  fold_conversions(&mut bytecode);
  jump_threading(&mut bytecode);
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
  let usage = analyze_usage(&bytecode);
  bytecode = eliminate_dead_stores(&bytecode, &usage)
    .into_iter()
    .map(Cow::into_owned)
    .collect();
  bytecode = eliminate_dead_loops(bytecode);
  let mut j = 0;
  while j < bytecode.len() {
    if let Instructions::Jump(target) = bytecode[j] {
      if target == j + 1 {
        bytecode[j] = Instructions::Nop;
      }
    }
    j += 1;
  }
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
  bytecode
}
