/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::vector::cross::{
  cross_f16in::cross_f16in, cross_f32in::cross_f32in, cross_f64in::cross_f64in,
  cross_i16in::cross_i16in, cross_i32in::cross_i32in, cross_i64in::cross_i64in,
  cross_i128in::cross_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
#[inline(always)]
pub fn cross_values(a_val: Value, b_val: Value, num_type: PrimitiveTypes) -> Value {
  let arr_a = match a_val.as_array() {
    Some(v) => v,
    None => return Value::NaN,
  };
  let arr_b = match b_val.as_array() {
    Some(v) => v,
    None => return Value::NaN,
  };
  if arr_a.len() != 3 || arr_b.len() != 3 {
    return Value::NaN;
  }
  let validator: fn(&Value) -> bool = match num_type {
    PrimitiveTypes::Sht => |v| matches!(v, Value::Int16(_)),
    PrimitiveTypes::Int => |v| matches!(v, Value::Int32(_)),
    PrimitiveTypes::Lng => |v| matches!(v, Value::Int64(_)),
    PrimitiveTypes::Oct => |v| matches!(v, Value::Int128(_)),
    PrimitiveTypes::Hlf => |v| matches!(v, Value::Float16(_)),
    PrimitiveTypes::Flt => |v| matches!(v, Value::Float32(_)),
    PrimitiveTypes::Dbl => |v| matches!(v, Value::Float64(_)),
    _ => return Value::NaN,
  };
  for x in arr_a.iter().chain(arr_b.iter()) {
    if !validator(x) {
      return Value::NaN;
    }
  }
  match num_type {
    PrimitiveTypes::Sht => cross_i16in(&arr_a, &arr_b),
    PrimitiveTypes::Int => cross_i32in(&arr_a, &arr_b),
    PrimitiveTypes::Lng => cross_i64in(&arr_a, &arr_b),
    PrimitiveTypes::Oct => cross_i128in(&arr_a, &arr_b),
    PrimitiveTypes::Hlf => cross_f16in(&arr_a, &arr_b),
    PrimitiveTypes::Flt => cross_f32in(&arr_a, &arr_b),
    PrimitiveTypes::Dbl => cross_f64in(&arr_a, &arr_b),
    _ => Value::NaN,
  }
}
#[inline]
pub fn cross_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let b_val = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "CROSS",
  })?;
  let a_ref = stack.last_mut().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "CROSS",
  })?;
  let a_val = std::mem::take(a_ref);
  *a_ref = cross_values(a_val, b_val, num_type);
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  use half::f16;
  use std::sync::Arc;
  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }
  fn ints(values: [i32; 3]) -> Value {
    array(values.into_iter().map(Value::Int32).collect())
  }
  #[test]
  fn cross_integer_vectors() {
    assert_eq!(
      cross_values(ints([1, 2, 3]), ints([4, 5, 6]), PrimitiveTypes::Int),
      ints([-3, 6, -3])
    );
  }
  #[test]
  fn cross_float_vectors_preserve_element_type() {
    let result = cross_values(
      array(vec![
        Value::Float32(1.0),
        Value::Float32(2.0),
        Value::Float32(3.0),
      ]),
      array(vec![
        Value::Float32(4.0),
        Value::Float32(5.0),
        Value::Float32(6.0),
      ]),
      PrimitiveTypes::Flt,
    );
    assert_eq!(
      result,
      array(vec![
        Value::Float32(-3.0),
        Value::Float32(6.0),
        Value::Float32(-3.0)
      ])
    );
    let result = cross_values(
      array(vec![
        Value::Float16(f16::ONE),
        Value::Float16(f16::from_f32(2.0)),
        Value::Float16(f16::from_f32(3.0)),
      ]),
      array(vec![
        Value::Float16(f16::from_f32(4.0)),
        Value::Float16(f16::from_f32(5.0)),
        Value::Float16(f16::from_f32(6.0)),
      ]),
      PrimitiveTypes::Hlf,
    );
    assert!(matches!(result.as_array().unwrap()[0], Value::Float16(_)));
  }
  #[test]
  fn cross_reversed_operands_negate_components() {
    assert_eq!(
      cross_values(ints([4, 5, 6]), ints([1, 2, 3]), PrimitiveTypes::Int),
      ints([3, -6, 3])
    );
  }
  #[test]
  fn cross_parallel_vectors_return_zero_vector() {
    assert_eq!(
      cross_values(ints([1, 2, 3]), ints([2, 4, 6]), PrimitiveTypes::Int),
      ints([0, 0, 0])
    );
  }
  #[test]
  fn cross_rejects_invalid_inputs() {
    assert_eq!(
      cross_values(Value::Int32(1), ints([1, 2, 3]), PrimitiveTypes::Int),
      Value::NaN
    );
    assert_eq!(
      cross_values(
        array(vec![Value::Int32(1)]),
        ints([1, 2, 3]),
        PrimitiveTypes::Int
      ),
      Value::NaN
    );
    assert_eq!(
      cross_values(
        array(vec![
          Value::Int32(1),
          Value::String("invalid".into()),
          Value::Int32(3)
        ]),
        ints([1, 2, 3]),
        PrimitiveTypes::Int,
      ),
      Value::NaN
    );
    assert_eq!(
      cross_values(ints([1, 2, 3]), ints([4, 5, 6]), PrimitiveTypes::Str),
      Value::NaN
    );
  }
  #[test]
  fn cross_reports_stack_underflow() {
    let mut stack = Stack::new();
    assert!(matches!(
      cross_func(&mut stack, PrimitiveTypes::Int, 7),
      Err(VMError::StackUnderflow {
        ip: 7,
        opcode: "CROSS"
      })
    ));
    stack.push(ints([1, 2, 3]));
    assert!(matches!(
      cross_func(&mut stack, PrimitiveTypes::Int, 8),
      Err(VMError::StackUnderflow {
        ip: 8,
        opcode: "CROSS"
      })
    ));
  }
}
