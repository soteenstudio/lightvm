/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

pub(crate) mod hyperbolic;
pub(crate) mod inverse;
#[cfg(test)]
fn assert_unary_float_vector_validation(
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
) {
  use crate::modules::vmerror::VMError;
  use crate::types::primitive_types::PrimitiveTypes;
  use crate::types::stack::Stack;
  use crate::types::value::Value;
  use std::sync::Arc;
  let array = |value| Value::Array(Arc::new(vec![value]));
  for (value, found) in [
    (Value::Int16(1), "Short"),
    (Value::Int32(1), "Integer"),
    (Value::Int64(1), "Long"),
    (Value::Int128(1), "Octa"),
    (Value::Bool(false), "Boolean"),
  ] {
    let operand = array(value);
    assert!(matches!(
      values(operand.clone(), PrimitiveTypes::Flt, 30),
      Err(VMError::TypeMismatch { ip: 30, expected: "Float", found: actual })
        if actual == found
    ));
    let mut stack = Stack::from_vec(vec![operand]);
    let original = stack.clone();
    assert!(matches!(
      func(&mut stack, PrimitiveTypes::Flt, 31),
      Err(VMError::TypeMismatch { ip: 31, expected: "Float", found: actual })
        if actual == found
    ));
    assert_eq!(stack, original);
  }
  for (value, num_type, expected) in [
    (
      Value::Float16(half::f16::ZERO),
      PrimitiveTypes::Flt,
      "Float",
    ),
    (Value::Float32(0.0), PrimitiveTypes::Dbl, "Double"),
    (Value::Float64(0.0), PrimitiveTypes::Hlf, "Half"),
  ] {
    let result = values(array(value), num_type, 32).unwrap();
    let elements = result.as_array().unwrap();
    assert_eq!(
      crate::utils::get_type_name::get_type_name(elements[0].clone()),
      expected
    );
  }
  let operand = array(Value::Float32(0.0));
  assert!(matches!(
    values(operand.clone(), PrimitiveTypes::Int, 33),
    Err(VMError::TypeMismatch {
      ip: 33,
      found: "unknown",
      ..
    })
  ));
  let mut stack = Stack::from_vec(vec![operand]);
  let original = stack.clone();
  assert!(matches!(
    func(&mut stack, PrimitiveTypes::Int, 34),
    Err(VMError::TypeMismatch {
      ip: 34,
      found: "unknown",
      ..
    })
  ));
  assert_eq!(stack, original);
}
