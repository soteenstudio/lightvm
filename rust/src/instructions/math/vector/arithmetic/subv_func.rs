/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::subv::{
  subv_f16in::subv_f16in, subv_f32in::subv_f32in, subv_f64in::subv_f64in, subv_i16in::subv_i16in,
  subv_i32in::subv_i32in, subv_i64in::subv_i64in, subv_i128in::subv_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn subv_values(
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
  if arr_a.len() != arr_b.len() || num_type == PrimitiveTypes::Str {
    return Ok(Value::NaN);
  }
  for element in arr_a.iter().chain(arr_b.iter()) {
    if !element.is_number() {
      return Err(element.type_of());
    }
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(subv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(subv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(subv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(subv_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Array(subv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(subv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(subv_f64in(&arr_a, &arr_b)),
    PrimitiveTypes::Str => Value::NaN,
  })
}
#[inline]
pub fn subv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "SUBV" });
  }
  let result = subv_values(
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
  fn subv_preserves_operand_order_and_wraps_integers() {
    assert_eq!(
      subv_values(
        array(vec![Value::Int32(5), Value::Int32(i32::MIN)]),
        array(vec![Value::Int32(8), Value::Int32(1)]),
        PrimitiveTypes::Int,
      ),
      Ok(array(vec![Value::Int32(-3), Value::Int32(i32::MAX)]))
    );
    assert_eq!(
      subv_values(
        array(vec![Value::Int128(i128::MIN)]),
        array(vec![Value::Int128(1)]),
        PrimitiveTypes::Oct,
      ),
      Ok(array(vec![Value::Int128(i128::MAX)]))
    );
  }
  #[test]
  fn subv_float_types_are_preserved() {
    let cases = [
      (
        PrimitiveTypes::Hlf,
        Value::Float16(f16::from_f32(3.0)),
        Value::Float16(f16::from_f32(1.0)),
        Value::Float16(f16::from_f32(2.0)),
      ),
      (
        PrimitiveTypes::Flt,
        Value::Float32(3.0),
        Value::Float32(1.0),
        Value::Float32(2.0),
      ),
      (
        PrimitiveTypes::Dbl,
        Value::Float64(3.0),
        Value::Float64(1.0),
        Value::Float64(2.0),
      ),
    ];
    for (num_type, a, b, expected) in cases {
      assert_eq!(
        subv_values(array(vec![a]), array(vec![b]), num_type),
        Ok(array(vec![expected]))
      );
    }
  }
  #[test]
  fn subv_structural_and_string_inputs_return_nan() {
    assert_eq!(
      subv_values(Value::Int32(1), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
    assert_eq!(
      subv_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int,
      ),
      Ok(Value::NaN)
    );
    assert_eq!(
      subv_values(array(vec![]), array(vec![]), PrimitiveTypes::Str),
      Ok(Value::NaN)
    );
  }
  #[test]
  fn subv_reports_element_type_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::String("invalid".into())]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      subv_func(&mut stack, PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn subv_reports_stack_underflow() {
    for mut stack in [Stack::new(), Stack::from_vec(vec![array(vec![])])] {
      assert!(matches!(
        subv_func(&mut stack, PrimitiveTypes::Int, 9),
        Err(VMError::StackUnderflow {
          ip: 9,
          opcode: "SUBV"
        })
      ));
    }
  }
}
