/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
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
  vars: &mut Vec<Value>,
  functions: &AHashMap<SmolStr, FuncMetadata>,
  options: &Option<RunOptions>,
  _current_ip: usize,
) {
  if let Some(opts) = &options {
    if let Some(entry_ip) = opts.entry {
      let entry_fn = functions.values().find(|f| f.start == entry_ip);
      if let Some(fn_meta) = entry_fn {
        for i in 0..fn_meta.params_count as usize {
          let val = opts.args.get(i).cloned().unwrap_or(Value::Undefined);
          if i < vars.len() {
            vars[i] = val;
          }
        }
      }
    }
  };
}
