/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::trigonometry::inverse::atan2v::{
  atan2v_f16in::atan2v_f16in, atan2v_f32in::atan2v_f32in, atan2v_f64in::atan2v_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn atan2v_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
) -> Result<Value, &'static str> {
  let arr_a = match a_val.as_array() {
    Some(v) => v,
    None => return Ok(Value::NaN),
  };
  let arr_b = match b_val.as_array() {
    Some(v) => v,
    None => return Ok(Value::NaN),
  };
  if num_type == PrimitiveTypes::Str {
    return Ok(Value::NaN);
  }
  for x in arr_a.iter().chain(arr_b.iter()) {
    if !x.is_number() {
      return Err(x.type_of());
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Array(atan2v_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(atan2v_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(atan2v_f64in(&arr_a, &arr_b)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn atan2v_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "ATAN2V",
    });
  }
  let result = atan2v_values(
    stack[stack.len() - 1].clone(),
    stack[stack.len() - 2].clone(),
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
    PrimitiveTypes::Sht => "Int16",
    PrimitiveTypes::Int => "Int32",
    PrimitiveTypes::Lng => "Int64",
    PrimitiveTypes::Oct => "Int128",
    PrimitiveTypes::Hlf => "Float16",
    PrimitiveTypes::Flt => "Float32",
    PrimitiveTypes::Dbl => "Float64",
    PrimitiveTypes::Str => "String",
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;
  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }
  #[test]
  fn atan2v_dbl_works_and_preserves_preceding_stack_values() {
    let expected = array(vec![Value::Float64(0.0)]);
    assert_eq!(
      atan2v_values(
        array(vec![Value::Float64(0.0)]),
        array(vec![Value::Float64(1.0)]),
        PrimitiveTypes::Dbl
      ),
      Ok(expected.clone())
    );
    let mut stack = Stack::from_vec(vec![
      Value::Bool(true),
      array(vec![Value::Float64(1.0)]),
      array(vec![Value::Float64(0.0)]),
    ]);
    atan2v_func(&mut stack, PrimitiveTypes::Dbl, 12).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::Bool(true), expected]));
  }
  #[test]
  fn atan2v_rejects_invalid_elements_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::String("invalid".into())]),
      array(vec![Value::Float64(0.0)]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      atan2v_func(&mut stack, PrimitiveTypes::Dbl, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Float64",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn atan2v_handles_non_array_input_and_underflow() {
    let mut non_array_stack =
      Stack::from_vec(vec![array(vec![Value::Float64(0.0)]), Value::Bool(false)]);
    atan2v_func(&mut non_array_stack, PrimitiveTypes::Dbl, 13).unwrap();
    assert_eq!(non_array_stack, Stack::from_vec(vec![Value::NaN]));
    let mut stack = Stack::from_vec(vec![Value::Bool(false)]);
    assert!(matches!(
      atan2v_func(&mut stack, PrimitiveTypes::Dbl, 14),
      Err(VMError::StackUnderflow {
        ip: 14,
        opcode: "ATAN2V"
      })
    ));
    assert_eq!(stack, Stack::from_vec(vec![Value::Bool(false)]));
    assert!(matches!(
      atan2v_func(&mut Stack::new(), PrimitiveTypes::Dbl, 15),
      Err(VMError::StackUnderflow {
        ip: 15,
        opcode: "ATAN2V"
      })
    ));
  }
}
