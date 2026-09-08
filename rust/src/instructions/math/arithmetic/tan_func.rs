/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::tan::{
  tan_f16in::tan_f16in, tan_f32in::tan_f32in, tan_f64in::tan_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::expected_category::ExpectedCategory;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::{expected_type::expected_type, get_type_name::get_type_name};
#[inline(always)]
pub fn tan_values(a: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
  if !a.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type, ExpectedCategory::Float),
      found: get_type_name(a),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Float16(tan_f16in(a.as_f16())),
    PrimitiveTypes::Flt => Value::Float32(tan_f32in(a.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(tan_f64in(a.as_f64())),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type, ExpectedCategory::Float),
        found: expected_type(num_type, ExpectedCategory::All),
      });
    }
  })
}
#[inline]
pub fn tan_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  let val = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode: "TAN" })?;
  let result = tan_values(val, num_type, ip)?;
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
      tan_values(invalid.clone(), PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Float",
        found: "string"
      })
    ));
    let mut stack = Stack::from_vec(vec![invalid]);
    let original = stack.clone();
    assert!(matches!(
      tan_func(&mut stack, PrimitiveTypes::Flt, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Float",
        found: "string"
      })
    ));
    assert_eq!(stack, original);
  }
}
