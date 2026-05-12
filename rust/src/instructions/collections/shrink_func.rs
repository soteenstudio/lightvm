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
#[inline(always)]
pub fn shrink_func(stack: &mut Vec<Value>) -> Result<(), SmolStr> {
  let len_val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on SHRINK (length)"))?;
  let target = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on SHRINK (target)"))?;
  let length = match len_val {
    Value::Int32(i) => i as usize,
    Value::Int64(i) => i as usize,
    _ => return Err(SmolStr::new("SHRINK length must be an integer")),
  };
  let result = match target {
    Value::String(s) => {
      let mut s_val = s.to_string();
      s_val.truncate(length);
      Value::String(SmolStr::from(s_val))
    }
    Value::Array(arr) => {
      let mut a_val = (*arr).clone();
      a_val.truncate(length);
      Value::Array(Arc::new(a_val))
    }
    _ => return Err(SmolStr::new("SHRINK target must be String or Array")),
  };
  stack.push(result);
  Ok(())
}
