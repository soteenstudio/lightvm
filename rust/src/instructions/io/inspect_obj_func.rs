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
use std::fmt::Write;
#[inline]
pub fn inspect_obj_func(stack: &mut [Value]) -> Result<(), SmolStr> {
  if let Some(top) = stack.last_mut() {
    if let Value::Object(obj) = top {
      let mut result = String::from("{ ");
      for (i, (k, v)) in obj.iter().enumerate() {
        if i > 0 {
          result.push_str(", ");
        }
        let _ = write!(result, "\"{}\": {:?}", k, v);
      }
      result.push_str(" }");
      *top = Value::String(SmolStr::from(result));
      Ok(())
    } else {
      Err(SmolStr::new("Value is not an object"))
    }
  } else {
    Err(SmolStr::new("Stack underflow on INSPECT_OBJ"))
  }
}
