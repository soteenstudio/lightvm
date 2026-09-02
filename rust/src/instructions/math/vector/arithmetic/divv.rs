/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use super::common::{array, arrays, binary_func};
use crate::instructions::math::arithmetic::div_func::div_values;
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};

pub fn divv_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  let Some((a, b)) = arrays(a, b) else {
    return Value::NaN;
  };
  array(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| div_values(a.clone(), b.clone(), num_type))
      .collect(),
  )
}

pub fn divv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  binary_func(stack, ip, "DIVV", |a, b| divv_values(a, b, num_type))
}
