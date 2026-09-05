/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::trigonometry::hyperbolic::sinhv::{
  sinhv_f16in::sinhv_f16in, sinhv_f32in::sinhv_f32in, sinhv_f64in::sinhv_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn sinhv_values(a_val: Value, num_type: PrimitiveTypes) -> Result<Value, &'static str> {
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
    PrimitiveTypes::Hlf => Value::Array(sinhv_f16in(&arr_a)),
    PrimitiveTypes::Flt => Value::Array(sinhv_f32in(&arr_a)),
    PrimitiveTypes::Dbl => Value::Array(sinhv_f64in(&arr_a)),
    _ => Value::NaN,
  })
}
#[inline]
pub fn sinhv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow {
      ip,
      opcode: "SINHV",
    });
  }
  let result = sinhv_values(stack[stack.len() - 1].clone(), num_type).map_err(|found| {
    VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type),
      found,
    }
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
  fn addv_i32_works() {
    let result = addv_values(
      array(vec![Value::Int32(1), Value::Int32(2)]),
      array(vec![Value::Int32(10), Value::Int32(20)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Ok(array(vec![Value::Int32(11), Value::Int32(22)])));
  }
  #[test]
  fn addv_rejects_non_numeric_and_mixed_or_invalid_lengths() {
    let result = addv_values(
      array(vec![Value::Int32(1)]),
      array(vec![Value::String("invalid".into())]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Err("string"));
    let result = addv_values(
      array(vec![Value::Int32(1)]),
      array(vec![Value::Int64(1)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Ok(array(vec![Value::Int32(2)])));
    let result = addv_values(
      array(vec![Value::Int32(1), Value::Int32(2)]),
      array(vec![Value::Int32(1)]),
      PrimitiveTypes::Int,
    );
    assert_eq!(result, Ok(Value::NaN));
  }
  #[test]
  fn addv_reports_element_type_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      addv_func(&mut stack, PrimitiveTypes::Int, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn addv_non_array_remains_nan() {
    assert_eq!(
      addv_values(Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
  }
}
