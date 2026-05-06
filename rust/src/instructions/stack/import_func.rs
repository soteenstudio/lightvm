/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::{RunOptions, Value};
use smol_str::SmolStr;
pub fn import_func(
  vars: &mut Vec<Value>,
  options: &Option<RunOptions>,
  module_name: &SmolStr,
  idx: usize,
) -> Result<(), SmolStr> {
  if let Some(opts) = options {
    if let Some(module_val) = opts.imports.get(module_name) {
      vars[idx] = module_val.clone();
      return Ok(());
    }
  }
  Err(SmolStr::new(format!("Module '{}' not found", module_name)))
}
