/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::{FuncMetadata, RunOptions, Value};
use ahash::AHashMap;
use smol_str::SmolStr;
#[inline]
pub fn inject_args(
  vars: &mut [Value],
  functions: &AHashMap<SmolStr, FuncMetadata>,
  options: &Option<RunOptions>,
  _current_ip: usize,
) {
  if let Some(opts) = options {
    if let Some(entry_ip) = opts.entry {
      if let Some(fn_meta) = functions.values().find(|f| f.start == entry_ip) {
        let params_to_fill = fn_meta.params_count as usize;
        let target_vars = if params_to_fill < vars.len() {
          &mut vars[..params_to_fill]
        } else {
          vars
        };
        for (i, slot) in target_vars.iter_mut().enumerate() {
          *slot = opts.args.get(i).cloned().unwrap_or(Value::Undefined);
        }
      }
    }
  }
}
