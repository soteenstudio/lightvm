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
#[inline]
#[cold]
pub fn eliminate_redundant_loads(bytecode: Vec<Instructions>) -> Vec<Instructions> {
  if bytecode.is_empty() {
    return bytecode;
  }
  let mut optimized = Vec::with_capacity(bytecode.len());
  for instr in bytecode {
    let is_redundant = if let Some(last) = optimized.last() {
      match (last, &instr) {
        (Instructions::Get(a), Instructions::Get(b)) if a == b => true,
        (Instructions::GetIdx(a), Instructions::GetIdx(b)) if a == b => true,
        (Instructions::Val(a), Instructions::Val(b)) if a == b => true,
        (Instructions::ValIdx(a), Instructions::ValIdx(b)) if a == b => true,
        (Instructions::Push(v1), Instructions::Push(v2)) if v1 == v2 => true,
        (Instructions::Set(a), Instructions::Get(b)) if a == b => true,
        (Instructions::SetIdx(a), Instructions::GetIdx(b)) if a == b => true,
        _ => false,
      }
    } else {
      false
    };
    if is_redundant {
      optimized.push(Instructions::Dup);
    } else {
      optimized.push(instr);
    }
  }
  optimized
}
