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
  divv::{
    divv_f16in::divv_f16in, divv_f32in::divv_f32in, divv_f64in::divv_f64in, divv_i16in::divv_i16in,
    divv_i32in::divv_i32in, divv_i64in::divv_i64in, divv_i128in::divv_i128in,
  },
  mulv_func::apply,
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};

#[inline(always)]
pub fn divv_values(
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
    PrimitiveTypes::Sht => Value::Array(divv_i16in(&arr_a, &arr_b)),
    PrimitiveTypes::Int => Value::Array(divv_i32in(&arr_a, &arr_b)),
    PrimitiveTypes::Lng => Value::Array(divv_i64in(&arr_a, &arr_b)),
    PrimitiveTypes::Oct => Value::Array(divv_i128in(&arr_a, &arr_b)),
    PrimitiveTypes::Hlf => Value::Array(divv_f16in(&arr_a, &arr_b)),
    PrimitiveTypes::Flt => Value::Array(divv_f32in(&arr_a, &arr_b)),
    PrimitiveTypes::Dbl => Value::Array(divv_f64in(&arr_a, &arr_b)),
    PrimitiveTypes::Str => Value::NaN,
  })
}

#[inline]
pub fn divv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  apply(stack, num_type, ip, "DIVV", divv_values)
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
  fn divv_preserves_order_handles_zero_and_supports_oct() {
    assert_eq!(
      divv_values(
        array(vec![Value::Int32(20), Value::Int32(7)]),
        array(vec![Value::Int32(4), Value::Int32(0)]),
        PrimitiveTypes::Int,
      ),
      Ok(array(vec![Value::Int32(5), Value::Int32(0)]))
    );
    assert_eq!(
      divv_values(
        array(vec![Value::Int128(21)]),
        array(vec![Value::Int128(3)]),
        PrimitiveTypes::Oct
      ),
      Ok(array(vec![Value::Int128(7)]))
    );
  }

  #[test]
  fn divv_preserves_float_types_and_zero_behavior() {
    let result = divv_values(
      array(vec![Value::Float32(1.0)]),
      array(vec![Value::Float32(0.0)]),
      PrimitiveTypes::Flt,
    )
    .unwrap();
    assert!(matches!(&(*result.as_array().unwrap())[..], [Value::Float32(value)] if value.is_nan()));
    assert_eq!(
      divv_values(
        array(vec![Value::Float16(f16::from_f32(4.0))]),
        array(vec![Value::Float16(f16::from_f32(2.0))]),
        PrimitiveTypes::Hlf,
      ),
      Ok(array(vec![Value::Float16(f16::from_f32(2.0))]))
    );
    assert_eq!(
      divv_values(
        array(vec![Value::Float64(4.0)]),
        array(vec![Value::Float64(2.0)]),
        PrimitiveTypes::Dbl
      ),
      Ok(array(vec![Value::Float64(2.0)]))
    );
  }

  #[test]
  fn divv_validates_structure_type_and_stack() {
    assert_eq!(
      divv_values(Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      Ok(Value::NaN)
    );
    assert_eq!(
      divv_values(
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int
      ),
      Ok(Value::NaN)
    );
    assert_eq!(
      divv_values(array(vec![]), array(vec![]), PrimitiveTypes::Str),
      Ok(Value::NaN)
    );
    let mut stack = Stack::from_vec(vec![
      array(vec![Value::Int32(1)]),
      array(vec![Value::Bool(false)]),
    ]);
    let original = stack.clone();
    assert!(matches!(
      divv_func(&mut stack, PrimitiveTypes::Int, 10),
      Err(VMError::TypeMismatch {
        ip: 10,
        expected: "Int32",
        found: "bool"
      })
    ));
    assert_eq!(stack, original);
    for mut stack in [Stack::new(), Stack::from_vec(vec![array(vec![])])] {
      assert!(matches!(
        divv_func(&mut stack, PrimitiveTypes::Int, 11),
        Err(VMError::StackUnderflow {
          ip: 11,
          opcode: "DIVV"
        })
      ));
    }
  }
}
