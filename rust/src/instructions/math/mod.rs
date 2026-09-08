/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

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
      found: "string"
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
