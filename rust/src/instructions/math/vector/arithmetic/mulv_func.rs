/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::mulv::{
  mulv_f16in::mulv_f16in, mulv_f32in::mulv_f32in, mulv_f64in::mulv_f64in, mulv_i16in::mulv_i16in,
  mulv_i32in::mulv_i32in, mulv_i64in::mulv_i64in, mulv_i128in::mulv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
#[inline(always)]
pub fn mulv_values(
  a_val: Value,
  b_val: Value,
  num_type: PrimitiveTypes,
) -> Result<Value, &'static str> {
  let Some(arr_a) = a_val.as_array() else {
    return Ok(Value::NaN);
  };
  let Some(arr_b) = b_val.as_array() else {
    return Ok(Value::NaN);
  };
  if arr_a.len() != arr_b.len() || num_type == PrimitiveTypes::Str {
    return Ok(Value::NaN);
  }
  if let Some(element) = arr_a
    .iter()
    .chain(arr_b.iter())
    .find(|value| !value.is_number())
  {
    return Err(element.type_of());
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(mulv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(mulv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(mulv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(mulv_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Array(mulv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(mulv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(mulv_f64in(&arr_a, &arr_b)),
    PrimitiveTypes::Str => Value::NaN,
  })
}
#[inline]
pub fn mulv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  apply(stack, num_type, ip, "MULV", mulv_values)
}
pub(super) fn apply(
  stack: &mut Stack,
  num_type: PrimitiveTypes,
  ip: usize,
  opcode: &'static str,
  operation: fn(Value, Value, PrimitiveTypes) -> Result<Value, &'static str>,
) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode });
  }
  let result = operation(
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
  use half::f16;
  use std::sync::Arc;
  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }
  #[test]
  fn mulv_integer_results_wrap_and_support_oct() {
    assert_eq!(
      mulv_values(
        array(vec![Value::Int32(3), Value::Int32(i32::MAX)]),
        array(vec![Value::Int32(4), Value::Int32(2)]),
        PrimitiveTypes::Int,
      ),
      Ok(array(vec![Value::Int32(12), Value::Int32(-2)]))
    );
    assert_eq!(
      mulv_values(
        array(vec![Value::Int128(i128::MAX)]),
        array(vec![Value::Int128(2)]),
        PrimitiveTypes::Oct,
      ),
      Ok(array(vec![Value::Int128(-2)]))
    );
  }
  #[test]
  fn mulv_preserves_float_output_types() {
    for (num_type, a, expected) in [
      (
        PrimitiveTypes::Hlf,
        Value::Float16(f16::from_f32(2.0)),
        Value::Float16(f16::from_f32(4.0)),
      ),
      (
        PrimitiveTypes::Flt,
        Value::Float32(2.0),
        Value::Float32(4.0),
      ),
      (
        PrimitiveTypes::Dbl,
        Value::Float64(2.0),
        Value::Float64(4.0),
      ),
    ] {
      assert_eq!(
        mulv_values(array(vec![a.clone()]), array(vec![a]), num_type),
        Ok(array(vec![expected]))
      );
    }
  }
  #[test]
  fn mulv_validates_structure_type_and_stack() {
    assert_eq!(
      mulv_values(Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
    assert_eq!(
      mulv_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int
      ),
      Ok(Value::NaN)
    );
    assert_eq!(
      mulv_values(array(vec![]), array(vec![]), PrimitiveTypes::Str),
      Ok(Value::NaN)
    );
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      mulv_func(&mut stack, PrimitiveTypes::Int, 8),
      Err(VMError::TypeMismatch {
        ip: 8,
        expected: "Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
    for mut stack in [Stack::new(), Stack::from_vec(vec![array(vec![])])] {
      assert!(matches!(
        mulv_func(&mut stack, PrimitiveTypes::Int, 9),
        Err(VMError::StackUnderflow {
          ip: 9,
          opcode: "MULV"
        })
      ));
    }
  }
}
