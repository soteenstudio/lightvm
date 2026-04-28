/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::{add_func::add_func, div_func::div_func};
use crate::optimizer::analyze_usage::analyze_usage;
use crate::optimizer::eliminate_dead_loops::eliminate_dead_loops;
use crate::optimizer::eliminate_dead_stores::eliminate_dead_stores;
use crate::types::instructions::Instructions;
use crate::types::value::Value;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(2) {
    if let (Instructions::Push(v1), Instructions::Push(v2)) = (&bytecode[i], &bytecode[i + 1]) {
      let mut folded_value: Option<Value> = None;
      match &bytecode[i + 2] {
        Instructions::Add(t) => folded_value = Some(add_func(v1.clone(), v2.clone(), t.clone())),
        Instructions::Div(t) => folded_value = Some(div_func(v1.clone(), v2.clone(), t.clone())),
        _ => {}
      }
      if let Some(result) = folded_value {
        bytecode.splice(i..i + 3, vec![Instructions::Push(result)]);
        i = i.saturating_sub(1);
        continue;
      }
    }
    i += 1;
  }
  let usage = analyze_usage(&bytecode);
  bytecode = eliminate_dead_stores(&bytecode, &usage);
  bytecode = eliminate_dead_loops(bytecode);
  let mut j = 0;
  while j < bytecode.len().saturating_sub(1) {
    if let Instructions::Jump(target) = bytecode[j] {
      if target == j + 1 {
        bytecode.remove(j);
        j = j.saturating_sub(1);
        continue;
      }
    }
    j += 1;
  }
  bytecode
}
