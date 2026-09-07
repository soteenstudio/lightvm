/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
pub fn get_type_name(num_type: Value) -> &'static str {
  match num_type {
    Value::String(_) => "string",
    Value::Array(_) => "array",
    Value::Object(_) => "object",
    Value::Null => "null",
    Value::Undefined => "undefined",
    Value::NaN => "nan",
    _ => "unknown",
  }
}
