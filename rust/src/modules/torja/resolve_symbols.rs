/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{instructions::Instructions, value::Value};
use ahash::AHashMap;
use smol_str::SmolStr;
#[inline(always)]
fn get_or_insert_idx(
  table: &mut AHashMap<SmolStr, usize>,
  name: &SmolStr,
  next_idx: &mut usize,
) -> usize {
  use std::collections::hash_map::Entry;
  match table.entry(name.clone()) {
    Entry::Occupied(o) => *o.get(),
    Entry::Vacant(v) => {
      let i = *next_idx;
      *next_idx += 1;
      *v.insert(i)
    }
  }
}
#[cold]
pub fn resolve_symbols(
  bytecode: &mut [Instructions],
  imports: &AHashMap<SmolStr, Value>,
) -> (usize, AHashMap<SmolStr, usize>) {
  let mut symbol_table: AHashMap<SmolStr, usize> = AHashMap::with_capacity(imports.len() + 16);
  let mut next_idx = 0;
  for name in imports.keys() {
    symbol_table.insert(name.clone(), next_idx);
    next_idx += 1;
  }
  for instr in bytecode.iter_mut() {
    match instr {
      Instructions::Get(name) => {
        let idx = get_or_insert_idx(&mut symbol_table, name, &mut next_idx);
        *instr = Instructions::GetIdx(idx);
      }
      Instructions::Set(name) => {
        let idx = get_or_insert_idx(&mut symbol_table, name, &mut next_idx);
        *instr = Instructions::SetIdx(idx);
      }
      Instructions::Inc(name, num_type) => {
        let idx = get_or_insert_idx(&mut symbol_table, name, &mut next_idx);
        *instr = Instructions::IncIdx(idx, *num_type);
      }
      Instructions::Func(_, _, _, _, names) => {
        for p_name in names {
          symbol_table.entry(p_name.clone()).or_insert_with(|| {
            let i = next_idx;
            next_idx += 1;
            i
          });
        }
      }
      _ => {}
    }
  }
  (next_idx, symbol_table)
}
