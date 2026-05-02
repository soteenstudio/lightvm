/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use super::eq_func::eq_func;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::value::Value;
#[inline]
pub fn neq_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  let is_equal = eq_func(a, b, num_type);
  if let Value::Bool(val) = is_equal {
    Value::Bool(!val)
  } else {
    Value::Bool(true)
  }
}
