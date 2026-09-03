/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::arithmetic::{
  modv_func::matches_type,
  negv::{
    negv_f16in::negv_f16in, negv_f32in::negv_f32in, negv_f64in::negv_f64in, negv_i16in::negv_i16in,
    negv_i32in::negv_i32in, negv_i64in::negv_i64in, negv_i128in::negv_i128in,
  },
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};

#[inline(always)]
pub fn negv_values(a_val: Value, num_type: PrimitiveTypes) -> Result<Value, &'static str> {
  let Some(arr) = a_val.as_array() else {
    return Ok(Value::NaN);
  };
  if num_type == PrimitiveTypes::Str {
    return Ok(Value::NaN);
  }
  if let Some(element) = arr.iter().find(|value| !matches_type(value, num_type)) {
    return Err(element.type_of());
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(negv_i16in(&arr)),
    PrimitiveTypes::Int => Value::Array(negv_i32in(&arr)),
    PrimitiveTypes::Lng => Value::Array(negv_i64in(&arr)),
    PrimitiveTypes::Oct => Value::Array(negv_i128in(&arr)),
    PrimitiveTypes::Hlf => Value::Array(negv_f16in(&arr)),
    PrimitiveTypes::Flt => Value::Array(negv_f32in(&arr)),
    PrimitiveTypes::Dbl => Value::Array(negv_f64in(&arr)),
    PrimitiveTypes::Str => Value::NaN,
  })
}

#[inline]
pub fn negv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let Some(value) = stack.last().cloned() else {
    return Err(VMError::StackUnderflow { ip, opcode: "NEGV" });
  };
  let result = negv_values(value, num_type).map_err(|found| VMError::TypeMismatch {
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
  use half::f16;
  use std::sync::Arc;

  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }

  #[test]
  fn negv_supports_all_types_and_boundaries() {
    let cases = [
      (
        PrimitiveTypes::Sht,
        Value::Int16(i16::MIN),
        Value::Int16(i16::MIN),
      ),
      (
        PrimitiveTypes::Int,
        Value::Int32(i32::MIN),
        Value::Int32(i32::MIN),
      ),
      (
        PrimitiveTypes::Lng,
        Value::Int64(i64::MIN),
        Value::Int64(i64::MIN),
      ),
      (
        PrimitiveTypes::Oct,
        Value::Int128(i128::MIN),
        Value::Int128(i128::MIN),
      ),
      (
        PrimitiveTypes::Hlf,
        Value::Float16(f16::from_f32(2.0)),
        Value::Float16(f16::from_f32(-2.0)),
      ),
      (
        PrimitiveTypes::Flt,
        Value::Float32(2.0),
        Value::Float32(-2.0),
      ),
      (
        PrimitiveTypes::Dbl,
        Value::Float64(2.0),
        Value::Float64(-2.0),
      ),
    ];
    for (num_type, value, expected) in cases {
      assert_eq!(
        negv_values(array(vec![value]), num_type),
        Ok(array(vec![expected]))
      );
    }
  }

  #[test]
  fn negv_validates_without_mutating_stack() {
    assert_eq!(
      negv_values(Value::Bool(false), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
    assert_eq!(
      negv_values(array(vec![]), PrimitiveTypes::Str),
      Ok(Value::NaN)
    );
    let mut stack = Stack::from_vec(vec![array(vec![Value::Bool(false)])]);
    let original = stack.clone();
    assert!(matches!(
      negv_func(&mut stack, PrimitiveTypes::Int, 9),
      Err(VMError::TypeMismatch {
        ip: 9,
        expected: "Int32",
        found: "bool"
      })
    ));
    assert_eq!(stack, original);
    assert!(matches!(
      negv_func(&mut Stack::new(), PrimitiveTypes::Int, 10),
      Err(VMError::StackUnderflow {
        ip: 10,
        opcode: "NEGV"
      })
    ));
  }
}
