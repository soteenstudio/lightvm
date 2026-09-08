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
    Value::Bool(_) => "bool",
    Value::Marker(_) => "marker",
    Value::Null => "null",
    Value::Undefined => "undefined",
    Value::NaN => "nan",
    _ => "unknown",
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn reports_bool_and_marker_types() {
    assert_eq!(get_type_name(Value::Bool(true)), "bool");
    assert_eq!(get_type_name(Value::Marker("label".into())), "marker");
  }
}
