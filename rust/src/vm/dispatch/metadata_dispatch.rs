/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::metadata::{length_func::length_func, typeof_func::typeof_func};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn metadata_dispatch(
  instr: &Instructions,
  stack: &mut SmallVec<[Value; 16]>,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::TypeOf => typeof_func(stack, ip),
    Instructions::Length => length_func(stack, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
