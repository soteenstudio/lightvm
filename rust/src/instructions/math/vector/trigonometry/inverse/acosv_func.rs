/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::trigonometry::inverse::acosv::{
  acosv_f16in::acosv_f16in, acosv_f32in::acosv_f32in, acosv_f64in::acosv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn acosv_values(a_val: Value, num_type: PrimitiveTypes) -> Result<Value, &'static str> {
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
    PrimitiveTypes::Hlf => Value::Array(acosv_f16in(&arr_a)),
    PrimitiveTypes::Flt => Value::Array(acosv_f32in(&arr_a)),
    PrimitiveTypes::Dbl => Value::Array(acosv_f64in(&arr_a)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn acosv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let Some(value) = stack.last().cloned() else {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "ACOSV",
    });
  };
  let result = acosv_values(value, num_type).map_err(|found| VMError::TypeMismatch {
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
  fn acosv_dbl_works_and_preserves_preceding_stack_values() {
    let expected = array(vec![Value::Float64(1.5707963267948966)]);
    assert_eq!(
      acosv_values(array(vec![Value::Float64(0.0)]), PrimitiveTypes::Dbl),
      Ok(expected.clone())
    );
    let mut stack = Stack::from_vec(vec![Value::Bool(true), array(vec![Value::Float64(0.0)])]);
    acosv_func(&mut stack, PrimitiveTypes::Dbl, 12).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::Bool(true), expected]));
  }
  #[test]
  fn acosv_rejects_invalid_elements_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      Value::Bool(true),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      acosv_func(&mut stack, PrimitiveTypes::Dbl, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Float64",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn acosv_handles_non_array_input_and_underflow() {
    let mut stack = Stack::from_vec(vec![Value::Bool(false)]);
    acosv_func(&mut stack, PrimitiveTypes::Dbl, 14).unwrap();
    assert_eq!(stack, Stack::from_vec(vec![Value::NaN]));
    assert!(matches!(
      acosv_func(&mut Stack::new(), PrimitiveTypes::Dbl, 15),
      Err(VMError::StackUnderflow {
        ip: 15,
        opcode: "ACOSV"
      })
    ));
  }
}
