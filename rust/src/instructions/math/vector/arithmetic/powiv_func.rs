/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::powiv::{
  powiv_f16in::powiv_f16in, powiv_f32in::powiv_f32in, powiv_f64in::powiv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn powiv_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
) -> Result<Value, &'static str> {
  let arr_a = match a_val.as_array() {
    Some(value) => value,
    None => return Ok(Value::NaN),
  };
  let arr_b = match b_val.as_array() {
    Some(value) => value,
    None => return Ok(Value::NaN),
  };
  if arr_a.len() != arr_b.len() {
    return Ok(Value::NaN);
  }
  if !matches!(
    num_type,
    PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl
  ) {
    return Ok(Value::NaN);
  }
  for value in arr_a.iter().chain(arr_b.iter()) {
    if !value.is_number() {
      return Err(value.type_of());
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Array(powiv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(powiv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(powiv_f64in(&arr_a, &arr_b)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn powiv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "POWIV",
    });
  }
  let result = powiv_values(
    stack[stack.len() - 2].clone(),
    stack[stack.len() - 1].clone(),
    num_type,
  )
  .map_err(|found| VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type),
    found,
  })?;
  stack.pop();
  *stack.last_mut().unwrap() = result;
  Ok(())
}
fn expected_type(num_type: PrimitiveTypes) -> &'static str {
  match num_type {
    PrimitiveTypes::Hlf => "Float16/Int16",
    PrimitiveTypes::Flt => "Float32/Int32",
    PrimitiveTypes::Dbl => "Float64/Int64",
    _ => "floating-point base/integer exponent",
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use half::f16;
  use std::sync::Arc;
  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }
  #[test]
  fn powiv_supports_each_precision() {
    for (num_type, bases, exponents, expected) in [
      (
        PrimitiveTypes::Hlf,
        array(vec![Value::Float16(f16::from_f32(2.0))]),
        array(vec![Value::Int16(3)]),
        array(vec![Value::Float16(f16::from_f32(8.0))]),
      ),
      (
        PrimitiveTypes::Flt,
        array(vec![Value::Float32(3.0)]),
        array(vec![Value::Int32(2)]),
        array(vec![Value::Float32(9.0)]),
      ),
      (
        PrimitiveTypes::Dbl,
        array(vec![Value::Float64(4.0)]),
        array(vec![Value::Int64(2)]),
        array(vec![Value::Float64(16.0)]),
      ),
    ] {
      assert_eq!(powiv_values(bases, exponents, num_type), Ok(expected));
    }
  }
  #[test]
  fn powiv_returns_nan_for_invalid_operands() {
    assert_eq!(
      powiv_values(Value::Bool(false), array(vec![]), PrimitiveTypes::Flt),
      Ok(Value::NaN)
    );
    assert_eq!(
      powiv_values(
        array(vec![Value::Float32(2.0)]),
        array(vec![]),
        PrimitiveTypes::Flt,
      ),
      Ok(Value::NaN)
    );
    assert_eq!(
      powiv_values(array(vec![]), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
  }
  #[test]
  fn powiv_reports_type_errors_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Float32(2.0)]),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      powiv_func(&mut stack, PrimitiveTypes::Flt, 11),
      Err(VMError::TypeMismatch {
        ip: 11,
        expected: "Float32/Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn powiv_reports_stack_underflow() {
    let mut stack = Stack::new();
    assert!(matches!(
      powiv_func(&mut stack, PrimitiveTypes::Flt, 12),
      Err(VMError::StackUnderflow {
        ip: 12,
        opcode: "POWIV"
      })
    ));
  }
}
