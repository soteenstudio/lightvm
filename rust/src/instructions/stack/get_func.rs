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
pub fn get_func(stack: &mut Vec<Value>, vars: &mut Vec<Value>, index: usize) {
  let val = vars.get(index).cloned().unwrap_or(Value::Undefined);
  stack.push(val);
}
