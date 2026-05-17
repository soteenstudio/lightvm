/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::math::{
  add_func::add_func,
  cos_func::cos_func,
  div_func::div_func,
  inc_dec::{dec_func, inc_func},
  mod_func::mod_func,
  mul_func::mul_func,
  pow_func::pow_func,
  powf_func::powf_func,
  powi_func::powi_func,
  rol_func::rol_func,
  ror_func::ror_func,
  shl_func::shl_func,
  shr_func::shr_func,
  sin_func::sin_func,
  sub_func::sub_func,
  tan_func::tan_func,
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
pub fn math_dispatch(
  instr: &Instructions,
  stack: &mut Vec<Value>,
  vars: &mut Vec<Value>,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::Add(num_type) => add_func(stack, *num_type, ip),
    Instructions::Sub(num_type) => sub_func(stack, *num_type, ip),
    Instructions::Mul(num_type) => mul_func(stack, *num_type, ip),
    Instructions::Div(num_type) => div_func(stack, *num_type, ip),
    Instructions::Mod(num_type) => mod_func(stack, *num_type, ip),
    Instructions::Shl(num_type) => shl_func(stack, *num_type, ip),
    Instructions::Shr(num_type) => shr_func(stack, *num_type, ip),
    Instructions::Ror(num_type) => ror_func(stack, *num_type, ip),
    Instructions::Rol(num_type) => rol_func(stack, *num_type, ip),
    Instructions::Pow(num_type) => pow_func(stack, *num_type, ip),
    Instructions::Powi(num_type) => powi_func(stack, *num_type, ip),
    Instructions::Powf(num_type) => powf_func(stack, *num_type, ip),
    Instructions::Sin(num_type) => sin_func(stack, *num_type, ip),
    Instructions::Cos(num_type) => cos_func(stack, *num_type, ip),
    Instructions::Tan(num_type) => tan_func(stack, *num_type, ip),
    Instructions::IncIdx(idx, num_type) => Ok(inc_func(vars, stack, *idx, *num_type, ip)?),
    Instructions::DecIdx(idx, num_type) => Ok(dec_func(vars, *idx, *num_type, ip)?),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
