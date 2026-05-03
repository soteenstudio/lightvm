/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use crate::types::usage::Usage;
#[inline]
pub fn eliminate_dead_stores(bytecode: &[Instructions], usage: &Usage) -> Vec<Instructions> {
  let mut result = Vec::new();
  let mut needed_by_next = 0;
  for inst in bytecode.iter().rev() {
    match inst {
      Instructions::Set(arg) => {
        if !usage.read.contains(arg.as_ref()) {
          continue;
        }
        needed_by_next += 1;
        result.push(inst.clone());
      }
      Instructions::Add(_) | Instructions::Sub(_) | Instructions::Mul(_) | Instructions::Div(_) => {
        if needed_by_next > 0 {
          needed_by_next -= 1;
          needed_by_next += 2;
          result.push(inst.clone());
        } else {
          continue;
        }
      }
      Instructions::Inc(arg, _) | Instructions::Dec(arg, _) => {
        if !usage.read.contains(arg.as_ref()) {
          continue;
        }
        result.push(inst.clone());
      }
      _ => {
        result.push(inst.clone());
      }
    }
  }
  result.reverse();
  result
}
