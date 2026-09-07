/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::add::{
  add_f16in::add_f16in, add_f32in::add_f32in, add_f64in::add_f64in, add_i16in::add_i16in,
  add_i32in::add_i32in, add_i64in::add_i64in, add_i128in::add_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;

#[inline(always)]
pub fn add_values(
  a: Value,
  b: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  if !a.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type),
      found: a.type_of(),
    });
  }
  if !b.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type),
      found: b.type_of(),
    });
  }

  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Int16(add_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(add_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(add_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(add_i128in(a.as_i128(), b.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(add_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(add_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(add_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Str => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type),
        found: a.type_of(),
      });
    }
  })
}

#[inline]
pub fn add_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "ADD" });
  }

  let b = stack.last().unwrap().clone();
  let a = stack[stack.len() - 2].clone();

  let result = add_values(a, b, num_type, ip)?;

  stack.pop();
  stack.pop();
  stack.push(result);
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

  #[test]
  fn add_reports_type_mismatch_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![Value::Int32(1), Value::String("invalid".into())]);
    let original = stack.clone();

    assert!(matches!(
      add_func(&mut stack, PrimitiveTypes::Int, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }

  #[test]
  fn add_values_reports_type_mismatch_for_string_left_operand() {
    assert!(matches!(
      add_values(
        Value::String("invalid".into()),
        Value::Int32(1),
        PrimitiveTypes::Int,
        21
      ),
      Err(VMError::TypeMismatch {
        ip: 21,
        expected: "Int32",
        found: "string"
      })
    ));
  }

  #[test]
  fn add_values_reports_type_mismatch_for_string_right_operand() {
    assert!(matches!(
      add_values(
        Value::Int32(1),
        Value::String("invalid".into()),
        PrimitiveTypes::Dbl,
        34
      ),
      Err(VMError::TypeMismatch {
        ip: 34,
        expected: "Float64",
        found: "string"
      })
    ));
  }
}
