/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
pub fn expected_type(num_type: PrimitiveTypes, category: ExpectedCategory) -> &'static str {
  match category {
    ExpectedCategory::Integer => match num_type {
      PrimitiveTypes::Hlf => "Half",
      PrimitiveTypes::Flt => "Float",
      PrimitiveTypes::Dbl => "Double",
      _ => "Unknown",
    },
    ExpectedCategory::Float => match num_type {
      PrimitiveTypes::Sht => "Short",
      PrimitiveTypes::Int => "Integer",
      PrimitiveTypes::Lng => "Long",
      PrimitiveTypes::Oct => "Octa",
      _ => "Unknown",
    },
    ExpectedCategory::All => match num_type {
      PrimitiveTypes::Sht => "Short",
      PrimitiveTypes::Int => "Integer",
      PrimitiveTypes::Lng => "Long",
      PrimitiveTypes::Oct => "Octa",
      PrimitiveTypes::Hlf => "Half",
      PrimitiveTypes::Flt => "Float",
      PrimitiveTypes::Dbl => "Double",
      PrimitiveTypes::Str => "String",
    },
  }
}
