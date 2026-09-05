/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::trigonometry::hyperbolic::inverse::atanhv::{
  atanhv_f16in::atanhv_f16in, atanhv_f32in::atanhv_f32in, atanhv_f64in::atanhv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn atanhv_values(a_val: Value, num_type: PrimitiveTypes) -> Result<Value, &'static str> {
  let arr_a = match a_val.as_array() {
    Some(v) => v,
    None => return Ok(Value::NaN),
  };
  if num_type == PrimitiveTypes::Str {
    return Ok(Value::NaN);
  }
  for x in arr_a.iter() {
    if !x.is_number() {
      return Err(x.type_of());
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Array(atanhv_f16in(&arr_a)),
    PrimitiveTypes::Flt => Value::Array(atanhv_f32in(&arr_a)),
    PrimitiveTypes::Dbl => Value::Array(atanhv_f64in(&arr_a)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn atanhv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let Some(value) = stack.last().cloned() else {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "ATANHV",
    });
  };
  let result = atanhv_values(value, num_type).map_err(|found| VMError::TypeMismatch {
    ip,
    expected: expected_type(num_type),
    found,
  })?;
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
  fn atanhv_dbl_works_and_preserves_preceding_stack_values() {
    let expected = array(vec![Value::Float64(0.0)]);
    assert_eq!(
      atanhv_values(array(vec![Value::Float64(0.0)]), PrimitiveTypes::Dbl),
      Ok(expected.clone())
    );
    let mut stack = Stack::from_vec(vec![Value::Bool(true), array(vec![Value::Float64(0.0)])]);
    atanhv_func(&mut stack, PrimitiveTypes::Dbl, 12).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::Bool(true), expected]));
  }
  #[test]
  fn atanhv_rejects_invalid_elements_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      Value::Bool(true),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      atanhv_func(&mut stack, PrimitiveTypes::Dbl, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Float64",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn atanhv_handles_non_array_input_and_underflow() {
    let mut stack = Stack::from_vec(vec![Value::Bool(false)]);
    atanhv_func(&mut stack, PrimitiveTypes::Dbl, 14).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::NaN]));
    assert!(matches!(
      atanhv_func(&mut Stack::new(), PrimitiveTypes::Dbl, 15),
      Err(VMError::StackUnderflow {
        ip: 15,
        opcode: "ATANHV"
      })
    ));
  }
}
