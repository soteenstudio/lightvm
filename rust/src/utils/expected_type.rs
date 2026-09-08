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
      PrimitiveTypes::Sht => "Short",
      PrimitiveTypes::Int => "Integer",
      PrimitiveTypes::Lng => "Long",
      PrimitiveTypes::Oct => "Octa",
      _ => "Integer",
    },
    ExpectedCategory::Float => match num_type {
      PrimitiveTypes::Hlf => "Half",
      PrimitiveTypes::Flt => "Float",
      PrimitiveTypes::Dbl => "Double",
      _ => "Float",
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
#[cfg(test)]
mod tests {
  use super::*;
  use crate::modules::vmerror::VMError;
  #[test]
  fn unsupported_directives_preserve_expected_category() {
    let expected = expected_type(PrimitiveTypes::Int, ExpectedCategory::Float);
    assert_eq!(expected, "Float");
    assert_eq!(
      expected_type(PrimitiveTypes::Flt, ExpectedCategory::Integer),
      "Integer"
    );
    let error = VMError::TypeMismatch {
      ip: 7,
      expected,
      found: "int32",
    };
    assert!(error.to_string().contains("Expected type 'Float'"));
  }
}
