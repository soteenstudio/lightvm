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
    Value::Int16(_) => "Short",
    Value::Int32(_) => "Integer",
    Value::Int64(_) => "Long",
    Value::Int128(_) => "Octa",
    Value::Float16(_) => "Half",
    Value::Float32(_) => "Float",
    Value::Float64(_) => "Double",
    Value::String(_) => "String",
    Value::Array(_) => "Array",
    Value::Object(_) => "Object",
    Value::Bool(_) => "Boolean",
    Value::Marker(_) => "Marker",
    Value::Null => "Null",
    Value::Undefined => "Undefined",
    Value::NaN => "NaN",
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
  #[test]
  fn reports_float_types() {
    assert_eq!(get_type_name(Value::Float16(half::f16::ONE)), "Half");
    assert_eq!(get_type_name(Value::Float32(1.0)), "Float");
    assert_eq!(get_type_name(Value::Float64(1.0)), "Double");
  }
  #[test]
  fn reports_integer_types() {
    assert_eq!(get_type_name(Value::Int16(1)), "Short");
    assert_eq!(get_type_name(Value::Int32(1)), "Integer");
    assert_eq!(get_type_name(Value::Int64(1)), "Long");
    assert_eq!(get_type_name(Value::Int128(1)), "Octa");
  }
}
