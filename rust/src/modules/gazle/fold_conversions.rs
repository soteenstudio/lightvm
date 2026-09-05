/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::{
  conversion::{
    to_double_func::to_double_values, to_float_func::to_float_values, to_half_func::to_half_values,
    to_integer_func::to_integer_values, to_long_func::to_long_values, to_octa_func::to_octa_values,
    to_short_func::to_short_values, to_string_func::to_string_values,
  },
  logic::not_func::not_values,
  math::{
    arithmetic::{
      cos_func::cos_values, neg_func::neg_values, sin_func::sin_values, tan_func::tan_values,
    },
    exp_func::exp_values,
    logarithm::ln_func::ln_values,
    root::{cbrt_func::cbrt_values, sqrt_func::sqrt_values},
    trigonometry::{
      hyperbolic::{
        cosh_func::cosh_values,
        inverse::{acosh_func::acosh_values, asinh_func::asinh_values, atanh_func::atanh_values},
        sinh_func::sinh_values,
        tanh_func::tanh_values,
      },
      inverse::{acos_func::acos_values, asin_func::asin_values, atan_func::atan_values},
    },
    vector::{
      arithmetic::{
        cosv_func::cosv_values, negv_func::negv_values, sinv_func::sinv_values,
        tanv_func::tanv_values,
      },
      trigonometry::hyperbolic::{
        coshv_func::coshv_values, sinhv_func::sinhv_values, tanhv_func::tanhv_values,
      },
    },
  },
  metadata::typeof_func::typeof_values,
};
use crate::modules::gazle::utils::{
  extract_value::extract_value, value_to_instruction::value_to_instruction,
};
use crate::types::instructions::Instructions;
#[inline(always)]
pub fn fold_conversions(bytecode: &mut [Instructions]) {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(1) {
    let instr1 = &bytecode[i];
    let instr2 = &bytecode[i + 1];
    if let Some(val) = extract_value(instr1) {
      let folded = match instr2 {
        Instructions::Not => Some(not_values(val)),
        Instructions::TypeOf => Some(typeof_values(val)),
        Instructions::ToShort => to_short_values(val).ok(),
        Instructions::ToInteger => to_integer_values(val).ok(),
        Instructions::ToLong => to_long_values(val).ok(),
        Instructions::ToOcta => to_octa_values(val).ok(),
        Instructions::ToHalf => to_half_values(val).ok(),
        Instructions::ToFloat => to_float_values(val).ok(),
        Instructions::ToDouble => to_double_values(val).ok(),
        Instructions::ToString => to_string_values(val).ok(),
        Instructions::Sin(t) => Some(sin_values(val, *t)),
        Instructions::Cos(t) => Some(cos_values(val, *t)),
        Instructions::Tan(t) => Some(tan_values(val, *t)),
        Instructions::Sinv(t) => sinv_values(val, *t).ok(),
        Instructions::Cosv(t) => cosv_values(val, *t).ok(),
        Instructions::Tanv(t) => tanv_values(val, *t).ok(),
        Instructions::Asin(t) => Some(asin_values(val, *t)),
        Instructions::Acos(t) => Some(acos_values(val, *t)),
        Instructions::Atan(t) => Some(atan_values(val, *t)),
        Instructions::Sinh(t) => Some(sinh_values(val, *t)),
        Instructions::Cosh(t) => Some(cosh_values(val, *t)),
        Instructions::Tanh(t) => Some(tanh_values(val, *t)),
        Instructions::Sinhv(t) => sinhv_values(val, *t).ok(),
        Instructions::Coshv(t) => coshv_values(val, *t).ok(),
        Instructions::Tanhv(t) => tanhv_values(val, *t).ok(),
        Instructions::Asinh(t) => Some(asinh_values(val, *t)),
        Instructions::Acosh(t) => Some(acosh_values(val, *t)),
        Instructions::Atanh(t) => Some(atanh_values(val, *t)),
        Instructions::Sqrt(t) => Some(sqrt_values(val, *t)),
        Instructions::Cbrt(t) => Some(cbrt_values(val, *t)),
        Instructions::Neg(t) => Some(neg_values(val, *t)),
        Instructions::Negv(t) => negv_values(val, *t).ok(),

        Instructions::Ln(t) => Some(ln_values(val, *t)),
        Instructions::Exp(t) => Some(exp_values(val, *t)),
        _ => None,
      };
      if let Some(res_val) = folded {
        bytecode[i] = value_to_instruction(res_val);
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
        continue;
      }
    }
    i += 1;
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::{primitive_types::PrimitiveTypes, value::Value};
  use std::sync::Arc;
  #[test]
  fn folds_valid_negv_and_leaves_invalid_negv_for_runtime() {
    let mut valid = vec![
      Instructions::PushArray(Arc::new(vec![Value::Int32(1), Value::Int32(i32::MIN)])),
      Instructions::Negv(PrimitiveTypes::Int),
    ];
    fold_conversions(&mut valid);
    assert_eq!(
      valid,
      vec![
        Instructions::PushArray(Arc::new(vec![Value::Int32(-1), Value::Int32(i32::MIN)])),
        Instructions::Nop,
      ]
    );
    let mut invalid = vec![
      Instructions::PushArray(Arc::new(vec![Value::String("invalid".into())])),
      Instructions::Negv(PrimitiveTypes::Int),
    ];
    let expected = invalid.clone();
    fold_conversions(&mut invalid);
    assert_eq!(invalid, expected);
  }
}
