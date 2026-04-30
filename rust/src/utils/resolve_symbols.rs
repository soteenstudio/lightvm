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
use std::collections::HashMap;
pub fn resolve_symbols(bytecode: &mut Vec<Instructions>) -> usize {
  let mut symbol_table: HashMap<String, usize> = HashMap::new();
  let mut next_idx = 0;
  for instr in bytecode.iter_mut() {
    match instr {
      Instructions::Get(name) => {
        let idx = *symbol_table.entry(name.clone()).or_insert_with(|| {
          let i = next_idx;
          next_idx += 1;
          i
        });
        *instr = Instructions::GetIdx(idx);
      }
      Instructions::Set(name) => {
        let idx = *symbol_table.entry(name.clone()).or_insert_with(|| {
          let i = next_idx;
          next_idx += 1;
          i
        });
        *instr = Instructions::SetIdx(idx);
      }
      Instructions::Inc(name, num_type) => {
        let idx = *symbol_table.entry(name.clone()).or_insert_with(|| {
          let i = next_idx;
          next_idx += 1;
          i
        });
        *instr = Instructions::IncIdx(idx, *num_type);
      }
      _ => {}
    }
  }
  next_idx
}
