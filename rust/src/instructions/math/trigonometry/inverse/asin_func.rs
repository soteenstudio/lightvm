/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::trigonometry::inverse::asin::{
  asin_f16in::asin_f16in, asin_f32in::asin_f32in, asin_f64in::asin_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
use half::f16;
use smol_str::SmolStr;
#[inline(always)]
pub fn asin_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  if !matches!(
    &a,
    Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(a.clone()),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => {
      let val = a.as_f16();
      if val.is_nan() || val < f16::from_f32(-1.0) || val > f16::from_f32(1.0) {
        return Err(VMError::ValueOutOfRange {
          ip,
          value: SmolStr::new(a.as_string()),
          min: SmolStr::new("-1.0"),
          max: SmolStr::new("1.0"),
        });
      }
      Value::Float16(asin_f16in(val))
    }
    PrimitiveTypes::Flt => {
      let val = a.as_f32();
      if val.is_nan() || val < -1.0 || val > 1.0 {
        return Err(VMError::ValueOutOfRange {
          ip,
          value: SmolStr::new(a.as_string()),
          min: SmolStr::new("-1.0"),
          max: SmolStr::new("1.0"),
        });
      }
      Value::Float32(asin_f32in(val))
    }
    PrimitiveTypes::Dbl => {
      let val = a.as_f64();
      if val.is_nan() || val < -1.0 || val > 1.0 {
        return Err(VMError::ValueOutOfRange {
          ip,
          value: SmolStr::new(a.as_string()),
          min: SmolStr::new("-1.0"),
          max: SmolStr::new("1.0"),
        });
      }
      Value::Float64(asin_f64in(val))
    }
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: "unknown",
      });
    }
  })
}
#[inline]
pub fn asin_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "ASIN" })?;
  let result = asin_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  #[test]
  fn reports_errors_without_mutating_stack() {
    crate::instructions::math::assert_unary_trigonometry_validation(
      super::asin_values,
      super::asin_func,
      "ASIN",
    );
  }
}
