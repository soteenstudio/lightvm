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
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
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
      expected: expected_type(num_type, ExpectedCategory::All),
      found: get_type_name(a),
    });
  }
  if !b.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::All),
      found: get_type_name(b),
    });
  }
  if matches!(
    num_type,
    PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct
  ) {
    for operand in [&a, &b] {
      if matches!(
        operand,
        &Value::Float16(_) | &Value::Float32(_) | &Value::Float64(_)
      ) {
        return Err(VMError::TypeMismatch {
          ip,
          expected: expected_type(num_type, ExpectedCategory::Integer),
          found: get_type_name(operand.clone()),
        });
      }
    }
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
        expected: expected_type(num_type, ExpectedCategory::All),
        found: get_type_name(a),
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
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn integer_directive_rejects_float_operands_without_mutating_stack() {
    for (a, b, num_type, found) in [
      (
        Value::Float16(half::f16::ONE),
        Value::Int16(2),
        PrimitiveTypes::Sht,
        "Half",
      ),
      (Value::Int32(1), Value::Float32(2.0), PrimitiveTypes::Int, "Float"),
      (Value::Float64(1.0), Value::Int64(2), PrimitiveTypes::Lng, "Double"),
      (Value::Int128(1), Value::Float64(2.0), PrimitiveTypes::Oct, "Double"),
    ] {
      assert!(matches!(
        add_values(a.clone(), b.clone(), num_type, 8),
        Err(VMError::TypeMismatch { ip: 8, expected: "Integer", found: actual })
          if actual == found
      ));
      let mut stack = Stack::from_vec(vec![a, b]);
      let original = stack.clone();
      assert!(matches!(
        add_func(&mut stack, num_type, 9),
        Err(VMError::TypeMismatch { ip: 9, expected: "Integer", found: actual })
          if actual == found
      ));
      assert_eq!(stack, original);
    }
  }
  #[test]
  fn add_reports_type_mismatch_without_mutating_stack() {
    let mut stack = Stack::from_vec(vec![Value::Int32(1), Value::String("invalid".into())]);
    let original = stack.clone();
    assert!(matches!(
      add_func(&mut stack, PrimitiveTypes::Int, 13),
      Err(VMError::TypeMismatch {
        ip: 13,
        expected: "Integer",
        found: "String"
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
        expected: "Integer",
        found: "String"
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
        expected: "Double",
        found: "String"
      })
    ));
  }
}
