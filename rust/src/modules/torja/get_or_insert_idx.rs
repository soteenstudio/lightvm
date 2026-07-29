/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use ahash::AHashMap;
use smol_str::SmolStr;
#[inline(always)]
pub fn get_or_insert_idx(
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
