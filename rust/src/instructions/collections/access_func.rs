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
pub fn access_func(stack: &mut Vec<Value>, prop: &SmolStr) -> Result<(), SmolStr> {
  let obj = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on ACCESS"))?;
  match obj {
    Value::Object(map) => {
      if let Some(val) = map.get(prop) {
        stack.push(val.clone());
        Ok(())
      } else {
        stack.push(Value::Undefined);
        Ok(())
      }
    }
    _ => Err(SmolStr::from(format!(
      "Cannot access property '{}' of non-object",
      prop
    ))),
  }
}
