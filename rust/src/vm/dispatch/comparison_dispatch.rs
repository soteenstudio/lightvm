/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::comparison::{
  eq_func::eq_func, ge_func::ge_func, gt_func::gt_func, le_func::le_func, lt_func::lt_func,
  neq_func::neq_func,
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
pub fn comparison_dispatch(
  instr: &Instructions,
  stack: &mut Vec<Value>,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::Gt(num_type) => gt_func(stack, *num_type, ip),
    Instructions::Lt(num_type) => lt_func(stack, *num_type, ip),
    Instructions::Ge(num_type) => ge_func(stack, *num_type, ip),
    Instructions::Le(num_type) => le_func(stack, *num_type, ip),
    Instructions::Eq(num_type) => eq_func(stack, *num_type, ip),
    Instructions::Neq(num_type) => neq_func(stack, *num_type, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
