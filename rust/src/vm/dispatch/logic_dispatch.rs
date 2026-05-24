/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::logic::{
  and_func::and_func, not_func::not_func, or_func::or_func, xor_func::xor_func,
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
pub fn logic_dispatch(
  instr: &Instructions,
  stack: &mut Vec<Value>,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::And => and_func(stack, ip),
    Instructions::Or => or_func(stack, ip),
    Instructions::Xor => xor_func(stack, ip),
    Instructions::Not => not_func(stack, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
