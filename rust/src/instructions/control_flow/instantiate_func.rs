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
use ahash::AHashMap;
use smol_str::SmolStr;
use std::sync::Arc;
pub fn instantiate_func(
  stack: &mut Vec<Value>,
  _vars: &mut Vec<Value>,
  class_name: &SmolStr,
  argc: u32,
) -> Result<Value, SmolStr> {
  let mut args = Vec::new();
  for _ in 0..argc {
    args.push(stack.pop().ok_or("Stack underflow on INSTANTIATE args")?);
  }
  args.reverse();
  let mut instance_map = AHashMap::new();
  instance_map.insert(SmolStr::new("__class"), Value::String(class_name.clone()));
  let instance = Value::Object(Arc::new(instance_map));
  Ok(instance)
}
