/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::trigonometry::inverse::atan2::{
  atan2_f16in::atan2_f16in, atan2_f32in::atan2_f32in, atan2_f64in::atan2_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn atan2_values(
  y: Value,
  x: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  if !matches!(
    &y,
    Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(y.clone()),
    });
  }
  if !matches!(
    &x,
    Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(x.clone()),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Float16(atan2_f16in(y.as_f16(), x.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(atan2_f32in(y.as_f32(), x.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(atan2_f64in(y.as_f64(), x.as_f64())),
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
pub fn atan2_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "ATAN2",
    });
  }
  let y = stack[stack.len() - 2].clone();
  let x = stack.last().unwrap().clone();
  let result = atan2_values(y, x, num_type, ip)?;
  stack.pop();
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn validates_float_operands_and_preserves_stack() {
    for num_type in [
      PrimitiveTypes::Hlf,
      PrimitiveTypes::Flt,
      PrimitiveTypes::Dbl,
    ] {
      for (invalid, found) in [
        (Value::Int16(1), "Short"),
        (Value::Int32(1), "Integer"),
        (Value::Int64(1), "Long"),
        (Value::Int128(1), "Octa"),
        (Value::Bool(false), "Boolean"),
      ] {
        for (y, x) in [
          (invalid.clone(), Value::Float32(1.0)),
          (Value::Float32(1.0), invalid.clone()),
        ] {
          assert!(matches!(
            atan2_values(y.clone(), x.clone(), num_type, 24),
            Err(VMError::TypeMismatch { ip: 24, found: actual, .. }) if actual == found
          ));
          let mut stack = Stack::from_vec(vec![y, x]);
          let original = stack.clone();
          assert!(matches!(
            atan2_func(&mut stack, num_type, 25),
            Err(VMError::TypeMismatch { ip: 25, found: actual, .. }) if actual == found
          ));
          assert_eq!(stack, original);
        }
      }
    }
    for (y, x, num_type, expected) in [
      (
        Value::Float16(half::f16::ONE),
        Value::Float64(1.0),
        PrimitiveTypes::Flt,
        "Float",
      ),
      (
        Value::Float32(1.0),
        Value::Float16(half::f16::ONE),
        PrimitiveTypes::Dbl,
        "Double",
      ),
      (
        Value::Float64(1.0),
        Value::Float32(1.0),
        PrimitiveTypes::Hlf,
        "Half",
      ),
    ] {
      let result = atan2_values(y, x, num_type, 26).unwrap();
      assert_eq!(get_type_name(result), expected);
    }
    let values = vec![Value::Float32(1.0), Value::Float32(1.0)];
    assert!(matches!(
      atan2_values(
        values[0].clone(),
        values[1].clone(),
        PrimitiveTypes::Int,
        27
      ),
      Err(VMError::TypeMismatch {
        ip: 27,
        found: "unknown",
        ..
      })
    ));
    let mut stack = Stack::from_vec(values);
    let original = stack.clone();
    assert!(matches!(
      atan2_func(&mut stack, PrimitiveTypes::Int, 28),
      Err(VMError::TypeMismatch {
        ip: 28,
        found: "unknown",
        ..
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn invalid_operands_preserve_stack() {
    for values in [
      vec![Value::String("invalid".into()), Value::Float32(1.0)],
      vec![Value::Float32(1.0), Value::String("invalid".into())],
    ] {
      let mut stack = Stack::from_vec(values);
      let original = stack.clone();
      assert!(matches!(
        atan2_func(&mut stack, PrimitiveTypes::Flt, 17),
        Err(VMError::TypeMismatch {
          ip: 17,
          expected: "Float",
          found: "String"
        })
      ));
      assert_eq!(stack, original);
    }
  }
  #[test]
  fn underflow_preserves_stack() {
    for values in [vec![], vec![Value::Float32(1.0)]] {
      let mut stack = Stack::from_vec(values);
      let original = stack.clone();
      assert!(matches!(
        atan2_func(&mut stack, PrimitiveTypes::Flt, 23),
        Err(VMError::StackUnderflow {
          ip: 23,
          opcode: "ATAN2"
        })
      ));
      assert_eq!(stack, original);
    }
  }
}
