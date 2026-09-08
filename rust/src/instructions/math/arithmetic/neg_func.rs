/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::neg::{
  neg_f16in::neg_f16in, neg_f32in::neg_f32in, neg_f64in::neg_f64in, neg_i16in::neg_i16in,
  neg_i32in::neg_i32in, neg_i64in::neg_i64in, neg_i128in::neg_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn neg_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  if !a.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::All),
      found: get_type_name(a),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Int16(neg_i16in(a.as_i16())),
    PrimitiveTypes::Int => Value::Int32(neg_i32in(a.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(neg_i64in(a.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(neg_i128in(a.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(neg_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(neg_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(neg_f64in(a.as_f64())),
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
pub fn neg_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let a = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "NEG" })?;
  let result = neg_values(a, num_type, ip)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn invalid_operand_reports_type_mismatch_and_preserves_stack() {
    let invalid = Value::String("invalid".into());
    assert!(matches!(
      neg_values(invalid.clone(), PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Int32",
        found: "string"
      })
    ));
    let mut stack = Stack::from_vec(vec![invalid]);
    let original = stack.clone();
    assert!(matches!(
      neg_func(&mut stack, PrimitiveTypes::Int, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Int32",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
}
