/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::optimizer::{
  analyze_usage::analyze_usage, eliminate_dead_loops::eliminate_dead_loops,
  eliminate_dead_stores::eliminate_dead_stores,
  eliminate_redundant_loads::eliminate_redundant_loads, fold_constants::fold_constants,
  fold_conversions::fold_conversions, jump_threading::jump_threading,
  strength_reduction::strength_reduction,
};
use crate::types::instructions::Instructions;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  strength_reduction(&mut bytecode);
  fold_constants(&mut bytecode);
  fold_conversions(&mut bytecode);
  jump_threading(&mut bytecode);
  let usage = analyze_usage(&bytecode);
  eliminate_dead_stores(&mut bytecode, &usage);
  bytecode = eliminate_dead_loops(bytecode);
  bytecode = eliminate_redundant_loads(bytecode);
  let mut current_idx = 0;
  bytecode.retain(|instr| {
    let keep = match instr {
      Instructions::Jump(target) => *target != current_idx + 1,
      Instructions::Nop => false,
      _ => true,
    };
    current_idx += 1;
    keep
  });
  bytecode
}
