/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::r#mod::{
  mod_f16in::mod_f16in, mod_f32in::mod_f32in, mod_f64in::mod_f64in, mod_i16in::mod_i16in,
  mod_i32in::mod_i32in, mod_i64in::mod_i64in, mod_i128in::mod_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn mod_values(
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
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Int16(mod_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(mod_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(mod_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(mod_i128in(a.as_i128(), b.as_i128())),
    PrimitiveTypes::Hlf => Value::Float16(mod_f16in(a.as_f16(), b.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(mod_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(mod_f64in(a.as_f64(), b.as_f64())),
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
pub fn mod_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "MOD" });
  }
  let result = mod_values(
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
  fn invalid_operands_report_type_mismatch_and_preserve_stack() {
    let invalid = Value::String("invalid".into());
    assert!(matches!(
      mod_values(invalid.clone(), Value::Int32(1), PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Integer",
        found: "string"
      })
    ));
    assert!(matches!(
      mod_values(Value::Int32(1), invalid.clone(), PrimitiveTypes::Int, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Integer",
        found: "string"
      })
    ));
    let mut stack = Stack::from_vec(vec![Value::Int32(1), invalid]);
    let original = stack.clone();
    assert!(matches!(
      mod_func(&mut stack, PrimitiveTypes::Int, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        expected: "Integer",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
}
