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
use crate::optimizer::{
  analyze_usage::analyze_usage, eliminate_dead_loops::eliminate_dead_loops,
  eliminate_dead_stores::eliminate_dead_stores, strength_reduction::strength_reduction,
};
use crate::types::{instructions::Instructions, value::Value};
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(2) {
    let mut instr1 = std::mem::replace(&mut bytecode[i], Instructions::Nop);
    let mut instr2 = std::mem::replace(&mut bytecode[i + 1], Instructions::Nop);
    let mut instr3 = std::mem::replace(&mut bytecode[i + 2], Instructions::Nop);
    match (&mut instr1, &mut instr2, &mut instr3) {
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Add(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = add_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Sub(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = sub_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Div(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = div_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mul(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = mul_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mod(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = mod_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      _ => {
        bytecode[i] = instr1;
        bytecode[i + 1] = instr2;
        bytecode[i + 2] = instr3;
        i += 1;
      }
    }
  }
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
  strength_reduction(&mut bytecode);
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
