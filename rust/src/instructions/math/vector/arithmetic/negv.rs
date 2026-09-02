/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use super::common::array;
use crate::instructions::math::arithmetic::neg_func::neg_values;
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};

pub fn negv_values(value: Value, num_type: PrimitiveTypes) -> Value {
  let Some(values) = value.as_array() else {
    return Value::NaN;
  };
  if !values.iter().all(Value::is_number) {
    return Value::NaN;
  }
  array(
    values
      .iter()
      .map(|value| neg_values(value.clone(), num_type))
      .collect(),
  )
}

pub fn negv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let value = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEGV" })?;
  *value = negv_values(std::mem::take(value), num_type);
  Ok(())
}
