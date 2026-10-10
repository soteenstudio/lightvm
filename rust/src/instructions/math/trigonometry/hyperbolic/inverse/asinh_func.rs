/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::trigonometry::hyperbolic::inverse::asinh::{
  asinh_f16in::asinh_f16in, asinh_f32in::asinh_f32in, asinh_f64in::asinh_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn asinh_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
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
      Value::Float16(asinh_f16in(val))
    }
    PrimitiveTypes::Flt => {
      let val = a.as_f32();
      Value::Float32(asinh_f32in(val))
    }
    PrimitiveTypes::Dbl => {
      let val = a.as_f64();
      Value::Float64(asinh_f64in(val))
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
pub fn asinh_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack.last().cloned().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "ASINH",
  })?;
  let result = asinh_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn accepts_asinh_inputs_outside_unit_interval() {
    for num_type in [
      PrimitiveTypes::Hlf,
      PrimitiveTypes::Flt,
      PrimitiveTypes::Dbl,
    ] {
      for input in [-10.0_f64, -2.0, -1.0, 0.0, 1.0, 2.0, 10.0] {
        let mut stack = Stack::from_vec(vec![Value::Float64(input)]);
        asinh_func(&mut stack, num_type, 31).unwrap();
        assert!((stack.last().unwrap().as_f64() - input.asinh()).abs() < 0.002);
      }
    }
  }
  #[test]
  fn reports_errors_without_mutating_stack() {
    crate::instructions::math::assert_unary_trigonometry_validation(
      super::asinh_values,
      super::asinh_func,
      "ASINH",
    );
  }
}
