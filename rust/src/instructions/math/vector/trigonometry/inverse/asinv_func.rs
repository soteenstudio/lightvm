/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::trigonometry::inverse::asinv::{
  asinv_f16in::asinv_f16in, asinv_f32in::asinv_f32in, asinv_f64in::asinv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn asinv_values(a_val: Value, num_type: PrimitiveTypes) -> Result<Value, &'static str> {
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
    PrimitiveTypes::Hlf => Value::Array(asinv_f16in(&arr_a)),
    PrimitiveTypes::Flt => Value::Array(asinv_f32in(&arr_a)),
    PrimitiveTypes::Dbl => Value::Array(asinv_f64in(&arr_a)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn asinv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let Some(value) = stack.last().cloned() else {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "ASINV",
    });
  };
  let result = asinv_values(value, num_type).map_err(|found| VMError::TypeMismatch {
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
  fn asinv_dbl_works_and_preserves_preceding_stack_values() {
    let expected = array(vec![Value::Float64(0.0)]);
    assert_eq!(
      asinv_values(array(vec![Value::Float64(0.0)]), PrimitiveTypes::Dbl),
      Ok(expected.clone())
    );
    let mut stack = Stack::from_vec(vec![Value::Bool(true), array(vec![Value::Float64(0.0)])]);
    asinv_func(&mut stack, PrimitiveTypes::Dbl, 12).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::Bool(true), expected]));
  }
  #[test]
  fn asinv_rejects_invalid_elements_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      Value::Bool(true),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      asinv_func(&mut stack, PrimitiveTypes::Dbl, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Float64",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn asinv_handles_non_array_input_and_underflow() {
    let mut stack = Stack::from_vec(vec![Value::Bool(false)]);
    asinv_func(&mut stack, PrimitiveTypes::Dbl, 14).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::NaN]));
    assert!(matches!(
      asinv_func(&mut Stack::new(), PrimitiveTypes::Dbl, 15),
      Err(VMError::StackUnderflow {
        ip: 15,
        opcode: "ASINV"
      })
    ));
  }
}
