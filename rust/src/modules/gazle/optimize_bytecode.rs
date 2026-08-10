/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::gazle::{
  analyze_usage::analyze_usage, constant_propagation::constant_propagation,
  eliminate_dead_loops::eliminate_dead_loops, eliminate_dead_stores::eliminate_dead_stores,
  eliminate_redundant_loads::eliminate_redundant_loads, fold_constants::fold_constants,
  fold_conversions::fold_conversions, jump_threading::jump_threading,
  specialized_instructions::specialized_instructions, strength_reduction::strength_reduction,
};
use crate::types::instructions::Instructions;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  loop {
    let previous_bytecode = bytecode.clone();
    specialized_instructions(&mut bytecode);
    strength_reduction(&mut bytecode);
    fold_constants(&mut bytecode);
    fold_conversions(&mut bytecode);
    jump_threading(&mut bytecode);
    constant_propagation(&mut bytecode);
    bytecode = eliminate_dead_loops(bytecode);
    bytecode = eliminate_redundant_loads(bytecode);
    let usage = analyze_usage(&bytecode);
    eliminate_dead_stores(&mut bytecode, &usage);
    let mut index_mapping = Vec::with_capacity(bytecode.len());
    let mut new_idx = 0;
    for (old_idx, instr) in bytecode.iter().enumerate() {
      let keep = match instr {
        Instructions::Jump(target) => *target != old_idx + 1,
        Instructions::Nop => false,
        _ => true,
      };
      if keep {
        index_mapping.push(new_idx);
        new_idx += 1;
      } else {
        index_mapping.push(new_idx);
      }
    }
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
    for instr in bytecode.iter_mut() {
      match instr {
        Instructions::Jump(target) | Instructions::IfFalse(target)
          if *target < index_mapping.len() =>
        {
          *target = index_mapping[*target];
        }
        _ => {}
      }
    }
    if bytecode == previous_bytecode {
      break;
    }
  }
  bytecode
}
