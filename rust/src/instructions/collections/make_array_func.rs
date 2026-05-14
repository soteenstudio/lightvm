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
#[inline]
pub fn make_array_func(stack: &mut Vec<Value>, count: u32) -> Result<(), SmolStr> {
  let count = count as usize;
  if stack.len() < count {
    return Err(SmolStr::new("Stack underflow: array element"));
  }
  let start_index = stack.len() - count;
  let elements: Vec<Value> = stack.drain(start_index..).collect();
  stack.push(Value::Array(Arc::new(elements)));
  Ok(())
}
