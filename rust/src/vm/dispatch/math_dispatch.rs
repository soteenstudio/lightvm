/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::{
  arithmetic::{
    add_func::add_func, cos_func::cos_func, div_func::div_func, mod_func::mod_func,
    mul_func::mul_func, neg_func::neg_func, pow_func::pow_func, powf_func::powf_func,
    powi_func::powi_func, sin_func::sin_func, sub_func::sub_func, tan_func::tan_func,
  },
  bitwise::{
    rotate::{rol_func::rol_func, ror_func::ror_func},
    shift::{shl_func::shl_func, shr_func::shr_func},
  },
  exp_func::exp_func,
  inc_dec::{dec_func, inc_func},
  logarithm::{ln_func::ln_func, log2_func::log2_func, log10_func::log10_func},
  root::{cbrt_func::cbrt_func, sqrt_func::sqrt_func},
  trigonometry::{
    hyperbolic::{
      cosh_func::cosh_func,
      inverse::{acosh_func::acosh_func, asinh_func::asinh_func, atanh_func::atanh_func},
      sinh_func::sinh_func,
      tanh_func::tanh_func,
    },
    inverse::{
      acos_func::acos_func, asin_func::asin_func, atan_func::atan_func, atan2_func::atan2_func,
    },
  },
  vector::{
    arithmetic::{
      addv_func::addv_func, cosv_func::cosv_func, divv_func::divv_func, modv_func::modv_func,
      mulv_func::mulv_func, negv_func::negv_func, sinv_func::sinv_func, subv_func::subv_func,
      tanv_func::tanv_func,
    },
    cross_func::cross_func,
    dot_func::dot_func,
    trigonometry::hyperbolic::{
      coshv_func::coshv_func, sinhv_func::sinhv_func, tanhv_func::tanhv_func,
    },
  },
};
use crate::modules::vmerror::VMError;
use crate::types::stack::Stack;
use crate::types::{instructions::Instructions, var_stack::VarStack};
#[inline(always)]
pub fn math_dispatch(
  instr: &Instructions,
  stack: &mut Stack,
  vars: &mut VarStack,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::Add(num_type) => add_func(stack, *num_type, ip),
    Instructions::Addv(num_type) => addv_func(stack, *num_type, ip),
    Instructions::Sub(num_type) => sub_func(stack, *num_type, ip),
    Instructions::Subv(num_type) => subv_func(stack, *num_type, ip),
    Instructions::Mul(num_type) => mul_func(stack, *num_type, ip),
    Instructions::Mulv(num_type) => mulv_func(stack, *num_type, ip),
    Instructions::Div(num_type) => div_func(stack, *num_type, ip),
    Instructions::Divv(num_type) => divv_func(stack, *num_type, ip),
    Instructions::Mod(num_type) => mod_func(stack, *num_type, ip),
    Instructions::Modv(num_type) => modv_func(stack, *num_type, ip),
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
    Instructions::Sinv(num_type) => sinv_func(stack, *num_type, ip),
    Instructions::Cosv(num_type) => cosv_func(stack, *num_type, ip),
    Instructions::Tanv(num_type) => tanv_func(stack, *num_type, ip),
    Instructions::Asin(num_type) => asin_func(stack, *num_type, ip),
    Instructions::Acos(num_type) => acos_func(stack, *num_type, ip),
    Instructions::Atan(num_type) => atan_func(stack, *num_type, ip),
    Instructions::Atan2(num_type) => atan2_func(stack, *num_type, ip),
    Instructions::Sinh(num_type) => sinh_func(stack, *num_type, ip),
    Instructions::Cosh(num_type) => cosh_func(stack, *num_type, ip),
    Instructions::Tanh(num_type) => tanh_func(stack, *num_type, ip),
    Instructions::Sinhv(num_type) => sinhv_func(stack, *num_type, ip),
    Instructions::Coshv(num_type) => coshv_func(stack, *num_type, ip),
    Instructions::Tanhv(num_type) => tanhv_func(stack, *num_type, ip),
    Instructions::Asinh(num_type) => asinh_func(stack, *num_type, ip),
    Instructions::Acosh(num_type) => acosh_func(stack, *num_type, ip),
    Instructions::Atanh(num_type) => atanh_func(stack, *num_type, ip),
    Instructions::Sqrt(num_type) => sqrt_func(stack, *num_type, ip),
    Instructions::Cbrt(num_type) => cbrt_func(stack, *num_type, ip),
    Instructions::Neg(num_type) => neg_func(stack, *num_type, ip),
    Instructions::Negv(num_type) => negv_func(stack, *num_type, ip),
    Instructions::Ln(num_type) => ln_func(stack, *num_type, ip),
    Instructions::Exp(num_type) => exp_func(stack, *num_type, ip),
    Instructions::Log2(num_type) => log2_func(stack, *num_type, ip),
    Instructions::Log10(num_type) => log10_func(stack, *num_type, ip),
    Instructions::Dot(num_type) => dot_func(stack, *num_type, ip),
    Instructions::Cross(num_type) => cross_func(stack, *num_type, ip),
    Instructions::IncIdx(idx, num_type) => Ok(inc_func(vars, stack, *idx, *num_type, ip)?),
    Instructions::DecIdx(idx, num_type) => Ok(dec_func(vars, *idx, *num_type, ip)?),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
