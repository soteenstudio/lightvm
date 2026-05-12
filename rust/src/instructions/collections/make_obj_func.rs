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
#[inline]
pub fn make_obj_func(stack: &mut Vec<Value>, count: u32) -> Result<(), SmolStr> {
  let mut obj = AHashMap::with_capacity(count as usize);
  for _ in 0..count {
    let val = stack
      .pop()
      .ok_or_else(|| SmolStr::new("Stack underflow: obj value"))?;
    let key_raw = stack
      .pop()
      .ok_or_else(|| SmolStr::new("Stack underflow: obj key"))?;
    if let Value::String(s) = key_raw {
      obj.insert(s, val);
    } else {
      return Err(SmolStr::new("TypeError: Object key must be a string"));
    }
  }
  stack.push(Value::Object(Arc::new(obj)));
  Ok(())
}
