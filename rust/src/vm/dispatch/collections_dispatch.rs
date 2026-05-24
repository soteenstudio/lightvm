/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::collections::{
  access_func::access_func, access_index_func::access_index_func, make_array_func::make_array_func,
  make_obj_func::make_obj_func, set_prop_func::set_prop_func, shrink_func::shrink_func,
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
pub fn collections_dispatch(
  instr: &Instructions,
  stack: &mut Vec<Value>,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::MakeObj(count) => make_obj_func(stack, *count, ip),
    Instructions::MakeArray(count) => make_array_func(stack, *count, ip),
    Instructions::AccessIndex => access_index_func(stack, ip),
    Instructions::Access(prop) => access_func(stack, prop, ip),
    Instructions::SetProp(prop) => set_prop_func(stack, prop, ip),
    Instructions::Shrink => shrink_func(stack, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
