/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::trigonometry::hyperbolic::inverse::acosh::{
  acosh_f16in::acosh_f16in, acosh_f32in::acosh_f32in, acosh_f64in::acosh_f64in,
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
pub fn acosh_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
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
      if val.is_nan() || val < f16::from_f32(1.0) {
        return Err(VMError::ValueOutOfRange {
          ip,
          value: SmolStr::new(a.as_string()),
          min: SmolStr::new("1.0"),
          max: SmolStr::new("inf"),
        });
      }
      Value::Float16(acosh_f16in(val))
    }
    PrimitiveTypes::Flt => {
      let val = a.as_f32();
      if val.is_nan() || val < 1.0 {
        return Err(VMError::ValueOutOfRange {
          ip,
          value: SmolStr::new(a.as_string()),
          min: SmolStr::new("1.0"),
          max: SmolStr::new("inf"),
        });
      }
      Value::Float32(acosh_f32in(val))
    }
    PrimitiveTypes::Dbl => {
      let val = a.as_f64();
      if val.is_nan() || val < 1.0 {
        return Err(VMError::ValueOutOfRange {
          ip,
          value: SmolStr::new(a.as_string()),
          min: SmolStr::new("1.0"),
          max: SmolStr::new("inf"),
        });
      }
      Value::Float64(acosh_f64in(val))
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
pub fn acosh_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack.last().cloned().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "ACOSH",
  })?;
  let result = acosh_values(val, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn validates_acosh_domain_in_all_precisions() {
    for num_type in [
      PrimitiveTypes::Hlf,
      PrimitiveTypes::Flt,
      PrimitiveTypes::Dbl,
    ] {
      for input in [-2.0, -1.0, 0.0, 0.5, f64::NEG_INFINITY] {
        let mut stack = Stack::from_vec(vec![Value::Bool(true), Value::Float64(input)]);
        let original = stack.clone();
        assert!(matches!(
          acosh_func(&mut stack, num_type, 31),
          Err(VMError::ValueOutOfRange { ip: 31, min, max, .. })
            if min == "1.0" && max == "inf"
        ));
        assert_eq!(stack, original);
      }
      for input in [1.0_f64, 2.0, 10.0] {
        let result = acosh_values(Value::Float64(input), num_type, 32).unwrap();
        assert!((result.as_f64() - input.acosh()).abs() < 0.002);
      }
      let result = acosh_values(Value::Float64(f64::INFINITY), num_type, 32).unwrap();
      assert_eq!(result.as_f64(), f64::INFINITY);
    }
  }
  #[test]
  fn reports_errors_without_mutating_stack() {
    crate::instructions::math::assert_unary_trigonometry_validation(
      super::acosh_values,
      super::acosh_func,
      "ACOSH",
    );
  }
}
