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
#[inline(always)]
pub fn length_func(stack: &mut Vec<Value>) {
  if let Some(val_ref) = stack.last_mut() {
    let len = match val_ref {
      Value::String(s) => s.len(),
      Value::Array(a) => a.len(),
      Value::Object(obj) => obj.len(),
      _ => 0,
    };
    *val_ref = Value::Int64(len as i64);
  }
}
