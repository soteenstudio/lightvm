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
  if !y.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(y),
    });
  }
  if !x.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(x),
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
        found: get_type_name(y),
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
          found: "string"
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
