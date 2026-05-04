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
pub fn inspect_arr_func(stack: &mut Vec<Value>) -> Result<(), SmolStr> {
  let val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on INSPECT_ARR"))?;
  if let Value::Array(arr) = val {
    let mut result = String::from("[");
    for (i, v) in arr.iter().enumerate() {
      if i > 0 {
        result.push_str(", ");
      }
      result.push_str(&format!("{:?}", v));
    }
    result.push(']');
    stack.push(Value::String(SmolStr::from(result)));
    Ok(())
  } else {
    Err(SmolStr::new("Value is not an array"))
  }
}
