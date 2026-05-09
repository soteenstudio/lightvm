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
use std::sync::Arc;
pub fn set_prop_func(stack: &mut Vec<Value>, prop: &SmolStr) -> Result<(), SmolStr> {
  let obj_val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on SET_PROP (object)"))?;
  let val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on SET_PROP (value)"))?;
  let mut obj = obj_val;
  if let Value::Object(ref mut map_arc) = obj {
    let map = Arc::make_mut(map_arc);
    map.insert(prop.clone(), val);
    stack.push(obj);
    Ok(())
  } else {
    Err(SmolStr::new("TypeError: Cannot set property of non-object"))
  }
}
