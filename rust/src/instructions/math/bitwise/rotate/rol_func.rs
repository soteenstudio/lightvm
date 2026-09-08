/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::bitwise::rotate::rol::{
  rol_i16in::rol_i16in, rol_i32in::rol_i32in, rol_i64in::rol_i64in, rol_i128in::rol_i128in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn rol_values(
  a: Value,
  b: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  if !a.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Integer),
      found: get_type_name(a),
    });
  }
  if !b.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Integer),
      found: get_type_name(b),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Sht => Value::Int16(rol_i16in(a.as_i16(), b.as_i16())),
    PrimitiveTypes::Int => Value::Int32(rol_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(rol_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Oct => Value::Int128(rol_i128in(a.as_i128(), b.as_i128())),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Integer),
        found: "unknown",
      });
    }
  })
}
#[inline]
pub fn rol_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "ROL" });
  }
  let result = rol_values(
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
      rol_values(invalid.clone(), Value::Int32(1), PrimitiveTypes::Int, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Integer",
        found: "string"
      })
    ));
    assert!(matches!(
      rol_values(Value::Int32(1), invalid.clone(), PrimitiveTypes::Int, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Integer",
        found: "string"
      })
    ));
    let mut stack = Stack::from_vec(vec![Value::Int32(1), invalid]);
    let original = stack.clone();
    assert!(matches!(
      rol_func(&mut stack, PrimitiveTypes::Int, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        expected: "Integer",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
}
