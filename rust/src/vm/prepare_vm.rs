/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{
  instructions::Instructions,
  value::{FuncMetadata, RunOptions},
};
use std::collections::HashMap;
use std::collections::HashSet;
pub fn prepare_vm(
  bytecode: &[Instructions],
  options: &Option<RunOptions>,
) -> (HashMap<String, FuncMetadata>, HashSet<String>, usize) {
  let mut functions = HashMap::new();
  let mut exported = HashSet::new();
  for instr in bytecode {
    if let Instructions::Func(name, params, start, end, names) = instr {
      functions.insert(
        name.clone(),
        FuncMetadata {
          params_count: *params,
          param_names: names.clone(),
          start: *start,
          end: *end,
        },
      );
    }
    if let Instructions::Export(name) = instr {
      exported.insert(name.clone());
    }
  }
  let ip = options.as_ref().and_then(|o| o.entry).unwrap_or(0);
  (functions, exported, ip)
}
