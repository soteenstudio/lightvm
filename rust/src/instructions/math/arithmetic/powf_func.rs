/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::powf::{
  powf_f16in::powf_f16in, powf_f32in::powf_f32in, powf_f64in::powf_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::expected_type::expected_type;
#[inline(always)]
pub fn powf_values(
  a: Value,
  b: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  let is_float = |value: &Value| {
    matches!(
      value,
      Value::Float16(_) | Value::Float32(_) | Value::Float64(_)
    )
  };
  if !is_float(&a) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: a.type_of(),
    });
  }
  if !is_float(&b) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: b.type_of(),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Float16(powf_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(powf_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(powf_f64in(a.as_f64(), b.as_f64())),
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
pub fn powf_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "POWF" });
  }
  let result = powf_values(
    stack[stack.len() - 2].clone(),
    stack.last().unwrap().clone(),
    num_type,
    ip,
  )?;
  stack.pop();
  stack.pop();
  stack.push(result);
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn invalid_operands_report_type_mismatch_and_preserve_stack() {
    let invalid = Value::String("invalid".into());
    assert!(matches!(
      powf_values(
        Value::Int32(1),
        Value::Float32(1.0),
        PrimitiveTypes::Flt,
        16
      ),
      Err(VMError::TypeMismatch {
        ip: 16,
        expected: "Float",
        found: "int32"
      })
    ));
    assert!(matches!(
      powf_values(invalid.clone(), Value::Int32(1), PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Float",
        found: "string"
      })
    ));
    assert!(matches!(
      powf_values(
        Value::Float32(1.0),
        invalid.clone(),
        PrimitiveTypes::Flt,
        18
      ),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Float",
        found: "string"
      })
    ));
    let mut stack = Stack::from_vec(vec![Value::Float32(1.0), invalid]);
    let original = stack.clone();
    assert!(matches!(
      powf_func(&mut stack, PrimitiveTypes::Flt, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        expected: "Float",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
    let mut stack = Stack::from_vec(vec![Value::Float32(1.0), Value::Int32(1)]);
    let original = stack.clone();
    assert!(matches!(
      powf_func(&mut stack, PrimitiveTypes::Flt, 20),
      Err(VMError::TypeMismatch {
        ip: 20,
        expected: "Float",
        found: "int32"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn accepts_cross_width_float_operands_and_returns_directive_type() {
    assert!(matches!(
      powf_values(
        Value::Float16(half::f16::from_f32(2.0)),
        Value::Float64(3.0),
        PrimitiveTypes::Flt,
        20
      ),
      Ok(Value::Float32(8.0))
    ));
    assert!(matches!(
      powf_values(
        Value::Float64(2.0),
        Value::Float32(3.0),
        PrimitiveTypes::Hlf,
        21
      ),
      Ok(Value::Float16(value)) if value == half::f16::from_f32(8.0)
    ));
    assert!(matches!(
      powf_values(
        Value::Float16(half::f16::from_f32(2.0)),
        Value::Float32(3.0),
        PrimitiveTypes::Dbl,
        22
      ),
      Ok(Value::Float64(8.0))
    ));
  }
}
