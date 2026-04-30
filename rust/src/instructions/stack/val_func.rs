/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use std::collections::HashMap;
pub fn val_func(vars: &mut HashMap<String, Value>, name: String) {
  vars
    .entry(name)
    .or_insert_with(|| Value::Marker("NoInitExpression".to_string()));
}
