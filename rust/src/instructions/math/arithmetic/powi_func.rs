/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::arithmetic::powi::{
  powi_f16in::powi_f16in, powi_f32in::powi_f32in, powi_f64in::powi_f64in,
};
use crate::modules::vmerror::VMError;
use crate::types::primitive_types::PrimitiveTypes;
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::get_type_name::get_type_name;
fn expected_type(num_type: PrimitiveTypes) -> &'static str {
  match num_type {
    PrimitiveTypes::Hlf => "Float16/Int16",
    PrimitiveTypes::Flt => "Float32/Int32",
    PrimitiveTypes::Dbl => "Float64/Int64",
    _ => "Float32/Int32",
  }
}
#[inline(always)]
pub fn powi_values(
  a: Value,
  b: Value,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<Value, VMError> {
  if !a.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type),
      found: get_type_name(a),
    });
  }
  if !b.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: expected_type(num_type),
      found: get_type_name(b),
    });
  }
  Ok(match num_type {
    PrimitiveTypes::Hlf => Value::Float16(powi_f16in(a.as_f16(), b.as_i16())),
    PrimitiveTypes::Flt => Value::Float32(powi_f32in(a.as_f32(), b.as_i32())),
    PrimitiveTypes::Dbl => Value::Float64(powi_f64in(a.as_f64(), b.as_i64())),
    _ => {
      return Err(VMError::TypeMismatch {
        ip,
        expected: expected_type(num_type),
        found: a.type_of(),
      });
    }
  })
}
#[inline]
pub fn powi_func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode: "POWI" });
  }
  let result = powi_values(
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
      powi_values(invalid.clone(), Value::Int32(1), PrimitiveTypes::Flt, 17),
      Err(VMError::TypeMismatch {
        ip: 17,
        expected: "Float32/Int32",
        found: "String"
      })
    ));
    assert!(matches!(
      powi_values(Value::Int32(1), invalid.clone(), PrimitiveTypes::Flt, 18),
      Err(VMError::TypeMismatch {
        ip: 18,
        expected: "Float32/Int32",
        found: "String"
      })
    ));
    let mut stack = Stack::from_vec(vec![Value::Int32(1), invalid]);
    let original = stack.clone();
    assert!(matches!(
      powi_func(&mut stack, PrimitiveTypes::Flt, 19),
      Err(VMError::TypeMismatch {
        ip: 19,
        expected: "Float32/Int32",
        found: "String"
      })
    ));
    assert_eq!(stack, original);
  }
  #[test]
  fn invalid_operands_report_directive_specific_expected_types() {
    let invalid = Value::String("invalid".into());
    assert!(matches!(
      powi_values(invalid.clone(), Value::Int16(1), PrimitiveTypes::Hlf, 20),
      Err(VMError::TypeMismatch {
        ip: 20,
        expected: "Float16/Int16",
        found: "String"
      })
    ));
    assert!(matches!(
      powi_values(Value::Float64(1.0), invalid, PrimitiveTypes::Dbl, 21),
      Err(VMError::TypeMismatch {
        ip: 21,
        expected: "Float64/Int64",
        found: "String"
      })
    ));
  }
  #[test]
  fn unsupported_directive_reports_default_expected_and_actual_operand_types() {
    assert!(matches!(
      powi_values(
        Value::Float64(2.0),
        Value::Int32(3),
        PrimitiveTypes::Int,
        22
      ),
      Err(VMError::TypeMismatch {
        ip: 22,
        expected: "Float32/Int32",
        found: "float64"
      })
    ));
  }
}
