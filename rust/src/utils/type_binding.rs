/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::primitive_types::PrimitiveTypes;
pub fn type_binding(num_type: &str) -> PrimitiveTypes {
  match num_type {
    "sht" => PrimitiveTypes::Sht,
    "int" => PrimitiveTypes::Int,
    "lng" => PrimitiveTypes::Lng,
    "oct" => PrimitiveTypes::Oct,
    "hlf" => PrimitiveTypes::Hlf,
    "flt" => PrimitiveTypes::Flt,
    "dbl" => PrimitiveTypes::Dbl,
    _ => PrimitiveTypes::Dbl,
  }
}
