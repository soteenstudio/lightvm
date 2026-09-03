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
  modv::{
    modv_f16in::modv_f16in, modv_f32in::modv_f32in, modv_f64in::modv_f64in, modv_i16in::modv_i16in,
    modv_i32in::modv_i32in, modv_i64in::modv_i64in, modv_i128in::modv_i128in,
  },
  mulv_func::apply,
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};

#[inline(always)]
pub fn modv_values(
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
    .find(|value| !matches_type(value, num_type))
  {
    return Err(element.type_of());
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Array(modv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(modv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(modv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(modv_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Array(modv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(modv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(modv_f64in(&arr_a, &arr_b)),
    PrimitiveTypes::Str => Value::NaN,
  })
}

#[inline]
pub fn modv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  apply(stack, num_type, ip, "MODV", modv_values)
}

pub(super) fn matches_type(value: &Value, num_type: PrimitiveTypes) -> bool {
  matches!(
    (value, num_type),
    (Value::Int16(_), PrimitiveTypes::Sht)
      | (Value::Int32(_), PrimitiveTypes::Int)
      | (Value::Int64(_), PrimitiveTypes::Lng)
      | (Value::Int128(_), PrimitiveTypes::Oct)
      | (Value::Float16(_), PrimitiveTypes::Hlf)
      | (Value::Float32(_), PrimitiveTypes::Flt)
      | (Value::Float64(_), PrimitiveTypes::Dbl)
  )
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
  fn modv_supports_all_types_and_scalar_edge_cases() {
    let cases = [
      (
        PrimitiveTypes::Sht,
        Value::Int16(7),
        Value::Int16(4),
        Value::Int16(3),
      ),
      (
        PrimitiveTypes::Int,
        Value::Int32(7),
        Value::Int32(4),
        Value::Int32(3),
      ),
      (
        PrimitiveTypes::Lng,
        Value::Int64(7),
        Value::Int64(4),
        Value::Int64(3),
      ),
      (
        PrimitiveTypes::Oct,
        Value::Int128(7),
        Value::Int128(4),
        Value::Int128(3),
      ),
      (
        PrimitiveTypes::Hlf,
        Value::Float16(f16::from_f32(7.0)),
        Value::Float16(f16::from_f32(4.0)),
        Value::Float16(f16::from_f32(3.0)),
      ),
      (
        PrimitiveTypes::Flt,
        Value::Float32(7.0),
        Value::Float32(4.0),
        Value::Float32(3.0),
      ),
      (
        PrimitiveTypes::Dbl,
        Value::Float64(7.0),
        Value::Float64(4.0),
        Value::Float64(3.0),
      ),
    ];
    for (num_type, left, right, expected) in cases {
      assert_eq!(
        modv_values(array(vec![left]), array(vec![right]), num_type),
        Ok(array(vec![expected]))
      );
    }
    assert_eq!(
      modv_values(
        array(vec![Value::Int32(7)]),
        array(vec![Value::Int32(0)]),
        PrimitiveTypes::Int
      ),
      Ok(array(vec![Value::Int32(0)]))
    );
    let result = modv_values(
      array(vec![Value::Float32(7.0)]),
      array(vec![Value::Float32(0.0)]),
      PrimitiveTypes::Flt,
    )
    .unwrap();
    assert!(matches!(&result.as_array().unwrap()[..], [Value::Float32(value)] if value.is_nan()));
  }

  #[test]
  fn modv_validates_without_mutating_stack() {
    assert_eq!(
      modv_values(Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
    assert_eq!(
      modv_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int
      ),
      Ok(Value::NaN)
    );
    assert_eq!(
      modv_values(array(vec![]), array(vec![]), PrimitiveTypes::Str),
      Ok(Value::NaN)
    );
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::Int64(1)]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      modv_func(&mut stack, PrimitiveTypes::Int, 7),
      Err(VMError::TypeMismatch {
        ip: 7,
        expected: "Int32",
        found: "int64"
      })
    ));
    assert_eq!(stack, original);
    for mut stack in [Stack::new(), Stack::from_vec(vec![array(vec![])])] {
      assert!(matches!(
        modv_func(&mut stack, PrimitiveTypes::Int, 8),
        Err(VMError::StackUnderflow {
          ip: 8,
          opcode: "MODV"
        })
      ));
    }
  }
}
