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
    Value::String(_) => "String",
    Value::Array(_) => "Array",
    Value::Object(_) => "Object",
    Value::Bool(_) => "Boolean",
    Value::Marker(_) => "Marker",
    Value::Null => "Null",
    Value::Undefined => "Undefined",
    Value::NaN => "NaN",
    _ => "Unknown",
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn reports_bool_and_marker_types() {
    assert_eq!(get_type_name(Value::Bool(true)), "Boolean");
    assert_eq!(get_type_name(Value::Marker("label".into())), "Marker");
  }
}
