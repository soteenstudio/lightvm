/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{value::Value, var_stack::VarStack};
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::sync::Arc;
pub fn instantiate_func(
  stack: &mut SmallVec<[Value; 16]>,
  _vars: &mut VarStack,
  class_name: &SmolStr,
  argc: u32,
  ip: usize,
) -> Result<Value, VMError> {
  let mut args = Vec::with_capacity(argc as usize);
  for _ in 0..argc {
    args.push(stack.pop().ok_or(VMError::StackUnderflow {
      ip,
      opcode: "INSTANTIATE",
    })?);
  }
  args.reverse();
  let mut instance_map = AHashMap::new();
  instance_map.insert(SmolStr::new("__class"), Value::String(class_name.clone()));
  let instance = Value::Object(Arc::new(instance_map));
  Ok(instance)
}
