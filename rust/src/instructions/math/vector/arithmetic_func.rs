/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::{
  add_func::add_values, div_func::div_values, mod_func::mod_values, mul_func::mul_values,
  neg_func::neg_values, sub_func::sub_values,
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use std::sync::Arc;

fn binary_values(
  a: Value,
  b: Value,
  num_type: PrimitiveTypes,
  operation: fn(Value, Value, PrimitiveTypes) -> Value,
) -> Value {
  if num_type == PrimitiveTypes::Str {
    return Value::NaN;
  }
  let (Some(a), Some(b)) = (a.as_array(), b.as_array()) else {
    return Value::NaN;
  };
  if a.len() != b.len() || !a.iter().chain(b.iter()).all(Value::is_number) {
    return Value::NaN;
  }
  Value::Array(Arc::new(
    a.iter()
      .zip(b.iter())
      .map(|(a, b)| operation(a.clone(), b.clone(), num_type))
      .collect(),
  ))
}

pub fn addv_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  binary_values(a, b, num_type, add_values)
}

pub fn subv_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  binary_values(a, b, num_type, sub_values)
}

pub fn mulv_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  binary_values(a, b, num_type, mul_values)
}

pub fn divv_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  binary_values(a, b, num_type, div_values)
}

pub fn modv_values(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  binary_values(a, b, num_type, mod_values)
}

pub fn negv_values(value: Value, num_type: PrimitiveTypes) -> Value {
  if num_type == PrimitiveTypes::Str {
    return Value::NaN;
  }
  let Some(values) = value.as_array() else {
    return Value::NaN;
  };
  if !values.iter().all(Value::is_number) {
    return Value::NaN;
  }
  Value::Array(Arc::new(
    values
      .iter()
      .map(|value| neg_values(value.clone(), num_type))
      .collect(),
  ))
}

fn binary_func(
  stack: &mut Stack,
  num_type: PrimitiveTypes,
  ip: usize,
  opcode: &'static str,
  operation: fn(Value, Value, PrimitiveTypes) -> Value,
) -> Result<(), VMError> {
  let b = stack.pop().ok_or(VMError::StackUnderflow { ip, opcode })?;
  let a = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode })?;
  *a = operation(std::mem::take(a), b, num_type);
  Ok(())
}

pub fn addv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  binary_func(stack, num_type, ip, "ADDV", addv_values)
}

pub fn subv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  binary_func(stack, num_type, ip, "SUBV", subv_values)
}

pub fn mulv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  binary_func(stack, num_type, ip, "MULV", mulv_values)
}

pub fn divv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  binary_func(stack, num_type, ip, "DIVV", divv_values)
}

pub fn modv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  binary_func(stack, num_type, ip, "MODV", modv_values)
}

pub fn negv_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let value = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEGV" })?;
  *value = negv_values(std::mem::take(value), num_type);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }

  fn ints(values: &[i32]) -> Value {
    array(values.iter().copied().map(Value::Int32).collect())
  }

  #[test]
  fn applies_integer_operations_in_operand_order() {
    assert_eq!(
      addv_values(ints(&[8, 9]), ints(&[2, 3]), PrimitiveTypes::Int),
      ints(&[10, 12])
    );
    assert_eq!(
      subv_values(ints(&[8, 9]), ints(&[2, 3]), PrimitiveTypes::Int),
      ints(&[6, 6])
    );
    assert_eq!(
      mulv_values(ints(&[8, 9]), ints(&[2, 3]), PrimitiveTypes::Int),
      ints(&[16, 27])
    );
    assert_eq!(
      divv_values(ints(&[8, 9]), ints(&[2, 3]), PrimitiveTypes::Int),
      ints(&[4, 3])
    );
    assert_eq!(
      modv_values(ints(&[8, 9]), ints(&[3, 2]), PrimitiveTypes::Int),
      ints(&[2, 1])
    );
    assert_eq!(
      negv_values(ints(&[8, -9]), PrimitiveTypes::Int),
      ints(&[-8, 9])
    );
  }

  #[test]
  fn preserves_scalar_edge_case_behavior() {
    assert_eq!(
      addv_values(ints(&[i32::MAX]), ints(&[1]), PrimitiveTypes::Int),
      ints(&[i32::MIN])
    );
    assert_eq!(
      divv_values(ints(&[8]), ints(&[0]), PrimitiveTypes::Int),
      ints(&[0])
    );
    assert_eq!(
      modv_values(ints(&[8]), ints(&[0]), PrimitiveTypes::Int),
      ints(&[0])
    );
    assert_eq!(
      subv_values(
        array(vec![Value::Int128(i128::MIN)]),
        array(vec![Value::Int128(1)]),
        PrimitiveTypes::Oct,
      ),
      array(vec![Value::Int128(i128::MAX)])
    );
    assert!(matches!(
      divv_values(
        array(vec![Value::Float32(1.0)]),
        array(vec![Value::Float32(0.0)]),
        PrimitiveTypes::Flt,
      )
      .as_array()
      .unwrap()[0],
      Value::Float32(value) if value.is_nan()
    ));
  }

  #[test]
  fn rejects_invalid_vector_inputs() {
    assert_eq!(
      addv_values(Value::Int32(1), ints(&[1]), PrimitiveTypes::Int),
      Value::NaN
    );
    assert_eq!(
      addv_values(ints(&[1]), ints(&[1, 2]), PrimitiveTypes::Int),
      Value::NaN
    );
    assert_eq!(
      addv_values(
        array(vec![Value::String("x".into())]),
        ints(&[1]),
        PrimitiveTypes::Int,
      ),
      Value::NaN
    );
    assert_eq!(
      addv_values(ints(&[1]), ints(&[2]), PrimitiveTypes::Str),
      Value::NaN
    );
    assert_eq!(
      negv_values(Value::Int32(1), PrimitiveTypes::Int),
      Value::NaN
    );
  }

  #[test]
  fn reports_opcode_specific_stack_underflow() {
    let mut stack = Stack::new();
    assert!(matches!(
      addv_func(&mut stack, PrimitiveTypes::Int, 3),
      Err(VMError::StackUnderflow {
        ip: 3,
        opcode: "ADDV"
      })
    ));
    assert!(matches!(
      negv_func(&mut stack, PrimitiveTypes::Int, 4),
      Err(VMError::StackUnderflow {
        ip: 4,
        opcode: "NEGV"
      })
    ));
  }
}
