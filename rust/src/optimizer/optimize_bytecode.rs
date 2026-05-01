/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::{
  add_func::add_func, div_func::div_func, mod_func::mod_func, mul_func::mul_func,
  sub_func::sub_func,
};
use crate::optimizer::analyze_usage::analyze_usage;
use crate::optimizer::eliminate_dead_loops::eliminate_dead_loops;
use crate::optimizer::eliminate_dead_stores::eliminate_dead_stores;
use crate::types::instructions::Instructions;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(2) {
    match (&bytecode[i], &bytecode[i + 1], &bytecode[i + 2]) {
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Add(t)) => {
        let result = add_func(v1.clone(), v2.clone(), *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 1;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Sub(t)) => {
        let result = sub_func(v1.clone(), v2.clone(), *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 1;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Div(t)) => {
        let result = div_func(v1.clone(), v2.clone(), *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 1;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mul(t)) => {
        let result = mul_func(v1.clone(), v2.clone(), *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 1;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mod(t)) => {
        let result = mod_func(v1.clone(), v2.clone(), *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 1;
      }
      _ => i += 1,
    }
  }
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
  let usage = analyze_usage(&bytecode);
  bytecode = eliminate_dead_stores(&bytecode, &usage);
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
