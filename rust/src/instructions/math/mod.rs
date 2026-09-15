/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

// TODO: math
pub(crate) mod arithmetic;
pub(crate) mod bitwise;
pub(crate) mod exp;
pub(crate) mod exp_func;
pub(crate) mod inc_dec;
pub(crate) mod logarithm;
pub(crate) mod root;
pub(crate) mod trigonometry;
pub(crate) mod vector;
#[cfg(test)]
fn assert_unary_trigonometry_validation(
  values: fn(
    crate::types::value::Value,
    crate::types::primitive_types::PrimitiveTypes,
    usize,
  ) -> Result<crate::types::value::Value, crate::modules::vmerror::VMError>,
  func: fn(
    &mut crate::types::stack::Stack,
    crate::types::primitive_types::PrimitiveTypes,
    usize,
  ) -> Result<(), crate::modules::vmerror::VMError>,
  opcode: &'static str,
) {
  use crate::modules::vmerror::VMError;
  use crate::types::primitive_types::PrimitiveTypes;
  use crate::types::stack::Stack;
  use crate::types::value::Value;
  for (num_type, expected) in [
    (PrimitiveTypes::Hlf, "Half"),
    (PrimitiveTypes::Flt, "Float"),
    (PrimitiveTypes::Dbl, "Double"),
  ] {
    for (value, found) in [
      (Value::Int16(1), "Short"),
      (Value::Int32(1), "Integer"),
      (Value::Int64(1), "Long"),
      (Value::Int128(1), "Octa"),
      (Value::String("invalid".into()), "String"),
    ] {
      assert!(matches!(
        values(value.clone(), num_type, 17),
        Err(VMError::TypeMismatch { ip: 17, expected: actual_expected, found: actual_found })
          if actual_expected == expected && actual_found == found
      ));
      let mut stack = Stack::from_vec(vec![value]);
      let original = stack.clone();
      assert!(matches!(
        func(&mut stack, num_type, 18),
        Err(VMError::TypeMismatch { ip: 18, expected: actual_expected, found: actual_found })
          if actual_expected == expected && actual_found == found
      ));
      assert_eq!(stack, original);
    }
  }
  for (value, num_type, result_matches) in [
    (
      Value::Float16(half::f16::ZERO),
      PrimitiveTypes::Flt,
      "Float",
    ),
    (Value::Float32(0.0), PrimitiveTypes::Dbl, "Double"),
    (Value::Float64(0.0), PrimitiveTypes::Hlf, "Half"),
  ] {
    let result = values(value, num_type, 19).unwrap();
    assert_eq!(
      crate::utils::get_type_name::get_type_name(result),
      result_matches
    );
  }
  let value = Value::Float32(0.0);
  assert!(matches!(
    values(value.clone(), PrimitiveTypes::Int, 20),
    Err(VMError::TypeMismatch {
      ip: 20,
      found: "unknown",
      ..
    })
  ));
  let mut stack = Stack::from_vec(vec![value]);
  let original = stack.clone();
  assert!(matches!(
    func(&mut stack, PrimitiveTypes::Int, 21),
    Err(VMError::TypeMismatch {
      ip: 21,
      found: "unknown",
      ..
    })
  ));
  assert_eq!(stack, original);
  let mut stack = Stack::new();
  assert!(matches!(
    func(&mut stack, PrimitiveTypes::Flt, 23),
    Err(VMError::StackUnderflow {
      ip: 23,
      opcode: found_opcode
    }) if found_opcode == opcode
  ));
}
#[cfg(test)]
fn assert_unary_float_errors(
  func: fn(
    &mut crate::types::stack::Stack,
    crate::types::primitive_types::PrimitiveTypes,
    usize,
  ) -> Result<(), crate::modules::vmerror::VMError>,
  opcode: &'static str,
) {
  use crate::modules::vmerror::VMError;
  use crate::types::primitive_types::PrimitiveTypes;
  use crate::types::stack::Stack;
  use crate::types::value::Value;
  let mut stack = Stack::from_vec(vec![Value::String("invalid".into())]);
  let original = stack.clone();
  assert!(matches!(
    func(&mut stack, PrimitiveTypes::Flt, 17),
    Err(VMError::TypeMismatch {
      ip: 17,
      expected: "Float",
      found: "String"
    })
  ));
  assert_eq!(stack, original);
  let mut stack = Stack::new();
  assert!(matches!(
    func(&mut stack, PrimitiveTypes::Flt, 23),
    Err(VMError::StackUnderflow {
      ip: 23,
      opcode: found_opcode
    }) if found_opcode == opcode
  ));
}
