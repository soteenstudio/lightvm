/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{
  instructions::Instructions,
  value::{FuncMetadata, RunOptions},
};
use ahash::AHashMap;
use smol_str::SmolStr;
use std::collections::HashSet;
#[inline]
#[cold]
pub fn prepare_vm(
  bytecode: &[Instructions],
  options: &Option<RunOptions>,
) -> (AHashMap<SmolStr, FuncMetadata>, HashSet<SmolStr>, usize) {
  let mut functions = AHashMap::with_capacity(8);
  let mut exported = HashSet::with_capacity(4);
  for instr in bytecode {
    match instr {
      Instructions::Func(name, params, start, end, names) => {
        functions.insert(
          name.clone(),
          FuncMetadata {
            params_count: *params,
            param_names: names.to_vec(),
            start: *start,
            end: *end,
          },
        );
      }
      Instructions::Export(name) => {
        exported.insert(name.clone());
      }
      _ => unsafe { std::hint::unreachable_unchecked() },
    }
  }
  let ip = options.as_ref().and_then(|o| o.entry).unwrap_or(0);
  (functions, exported, ip)
}
