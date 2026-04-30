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
pub fn set_func(stack: &mut Vec<Value>, vars: &mut HashMap<String, Value>, name: String) {
  if let Some(val) = stack.pop() {
    vars.insert(name, val);
    if stack.len() > 50 {
      stack.truncate(50);
    }
  } else {
    panic!("Stack underflow on SET");
  }
}
