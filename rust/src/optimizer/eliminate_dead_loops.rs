/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::optimizer::is_pure_loop::is_pure_loop;
use crate::types::instructions::Instructions;
use ahash::AHashMap;
#[inline(always)]
pub fn eliminate_dead_loops(bytecode: Vec<Instructions>) -> Vec<Instructions> {
  let mut out: Vec<Instructions> = Vec::new();
  let mut index_map: AHashMap<usize, usize> = AHashMap::new();
  for (i, inst) in bytecode.iter().enumerate() {
    let mut is_eliminated = false;
    match inst {
      Instructions::Jump(target) | Instructions::IfFalse(target) if *target < i => {
        let loop_start = *target;
        let loop_end = i;
        if is_pure_loop(&bytecode, loop_start, loop_end) {
          if let Some(&out_start_index) = index_map.get(&loop_start) {
            out.truncate(out_start_index);
            is_eliminated = true;
          }
        }
      }
      _ => {}
    }
    if is_eliminated {
      continue;
    }
    index_map.insert(i, out.len());
    out.push(inst.clone());
  }
  out
}
