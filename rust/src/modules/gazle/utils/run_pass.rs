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
#[inline(always)]
pub fn run_pass(pass_id: usize, bytecode: &mut Vec<Instructions>) {
  match pass_id {
    0 => specialized_instructions(bytecode),
    1 => strength_reduction(bytecode),
    2 => fold_constants(bytecode),
    3 => fold_conversions(bytecode),
    4 => jump_threading(bytecode),
    5 => constant_propagation(bytecode),
    6 => {
      let taken = std::mem::take(bytecode);
      *bytecode = eliminate_dead_loops(taken);
    }
    7 => {
      let taken = std::mem::take(bytecode);
      *bytecode = eliminate_redundant_loads(taken);
    }
    8 => {
      let usage = analyze_usage(bytecode);
      eliminate_dead_stores(bytecode, &usage);
    }
    _ => {}
  }
}
