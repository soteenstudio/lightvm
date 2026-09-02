/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::vmerror::VMError;
use crate::types::stack::Stack;
use crate::types::value::Value;
use std::sync::Arc;

pub(super) fn arrays(a: Value, b: Value) -> Option<(Arc<Vec<Value>>, Arc<Vec<Value>>)> {
  let a = a.as_array()?;
  let b = b.as_array()?;
  if a.len() != b.len() || !a.iter().chain(b.iter()).all(Value::is_number) {
    return None;
  }
  Some((a, b))
}

pub(super) fn array(values: Vec<Value>) -> Value {
  Value::Array(Arc::new(values))
}

pub(super) fn binary_func(
  stack: &mut Stack,
  ip: usize,
  opcode: &'static str,
  operation: impl FnOnce(Value, Value) -> Value,
) -> Result<(), VMError> {
  let b = stack.pop().ok_or(VMError::StackUnderflow { ip, opcode })?;
  let a_ref = stack
    .last_mut()
    .ok_or(VMError::StackUnderflow { ip, opcode })?;
  let a = std::mem::take(a_ref);
  *a_ref = operation(a, b);
  Ok(())
}
