/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::mul::{
  mul_f16in::mul_f16in, mul_f32in::mul_f32in, mul_f64in::mul_f64in, mul_i16in::mul_i16in,
  mul_i32in::mul_i32in, mul_i64in::mul_i64in, mul_i128in::mul_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn mul_values(
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
    PrimitiveTypes::Sht => Value::Int16(mul_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(mul_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(mul_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(mul_i128in(a.as_i128(), b.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(mul_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(mul_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(mul_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Str => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: "number",
        found: expected_type(num_type, ExpectedCategory::All),
      });
    }
  })
}
#[inline]
pub fn mul_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "MUL" });
  }
  let result = mul_values(
    stack[stack.len() - 2].clone(),
    stack.last().unwrap().clone(),
    num_type,
    ip,
  )?;
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
    for (a, b, found) in [
      (Value::Float32(1.0), Value::Int32(2), "Float"),
      (Value::Int32(1), Value::Float64(2.0), "Double"),
    ] {
      assert!(matches!(
        mul_values(a.clone(), b.clone(), PrimitiveTypes::Int, 8),
        Err(VMError::TypeMismatch { ip: 8, expected: "Integer", found: actual }) if actual == found
      ));
      let mut stack = Stack::from_vec(vec![a, b]);
      let original = stack.clone();
      assert!(matches!(
        mul_func(&mut stack, PrimitiveTypes::Int, 9),
        Err(VMError::TypeMismatch { ip: 9, expected: "Integer", found: actual }) if actual == found
      ));
      assert_eq!(stack, original);
    }
  }
  #[test]
  fn invalid_operands_report_type_mismatch_and_preserve_stack() {
    let invalid = Value::String("invalid".into());
    assert!(matches!(
      mul_values(invalid.clone(), Value::Int32(1), PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Integer",
        found: "String"
      })
    ));
    assert!(matches!(
      mul_values(Value::Int32(1), invalid.clone(), PrimitiveTypes::Int, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Integer",
        found: "String"
      })
    ));
    let mut stack = Stack::from_vec(vec![Value::Int32(1), invalid]);
    let original = stack.clone();
    assert!(matches!(
      mul_func(&mut stack, PrimitiveTypes::Int, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        expected: "Integer",
        found: "String"
      })
    ));
    assert_eq!(stack, original);
  }
}
