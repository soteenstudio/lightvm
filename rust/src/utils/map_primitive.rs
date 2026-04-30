/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::primitive_types::PrimitiveTypes;
use serde_json::Value as JsonValue;
pub fn map_primitive(val: Option<&JsonValue>) -> PrimitiveTypes {
  match val.and_then(|v| v.as_str()) {
    Some("int") => PrimitiveTypes::Int,
    Some("lng") => PrimitiveTypes::Lng,
    Some("flt") => PrimitiveTypes::Flt,
    Some("dbl") => PrimitiveTypes::Dbl,
    Some("str") => PrimitiveTypes::Str,
    _ => PrimitiveTypes::Int,
  }
}
