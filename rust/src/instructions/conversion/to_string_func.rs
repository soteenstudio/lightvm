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
use smol_str::SmolStr;
pub fn to_string_func(stack: &mut Vec<Value>) {
  if let Some(val) = stack.pop() {
    let s = match val {
      Value::String(s) => s.to_string(),
      _ => val.to_string(),
    };
    stack.push(Value::String(SmolStr::new(format!("{}::string", s))));
  }
}
