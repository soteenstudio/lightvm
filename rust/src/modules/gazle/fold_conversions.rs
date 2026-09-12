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
      logarithm::{
        expv_func::expv_values, lnv_func::lnv_values, log2v_func::log2v_values,
        log10v_func::log10v_values,
      },
      normalize::normalize_func::normalize_values,
      root::{cbrtv_func::cbrtv_values, sqrtv_func::sqrtv_values},
      trigonometry::{
        hyperbolic::{
          coshv_func::coshv_values,
          inverse::{
            acoshv_func::acoshv_values, asinhv_func::asinhv_values, atanhv_func::atanhv_values,
          },
          sinhv_func::sinhv_values,
          tanhv_func::tanhv_values,
        },
        inverse::{acosv_func::acosv_values, asinv_func::asinv_values, atanv_func::atanv_values},
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
        Instructions::Sin(t) => sin_values(val, *t, i).ok(),
        Instructions::Cos(t) => cos_values(val, *t, i).ok(),
        Instructions::Tan(t) => tan_values(val, *t, i).ok(),
        Instructions::Sinv(t) => sinv_values(val, *t, i).ok(),
        Instructions::Cosv(t) => cosv_values(val, *t, i).ok(),
        Instructions::Tanv(t) => tanv_values(val, *t, i).ok(),
        Instructions::Asin(t) => asin_values(val, *t, i).ok(),
        Instructions::Acos(t) => acos_values(val, *t, i).ok(),
        Instructions::Atan(t) => atan_values(val, *t, i).ok(),
        Instructions::Sinh(t) => sinh_values(val, *t, i).ok(),
        Instructions::Cosh(t) => cosh_values(val, *t, i).ok(),
        Instructions::Tanh(t) => tanh_values(val, *t, i).ok(),
        Instructions::Sinhv(t) => sinhv_values(val, *t, i).ok(),
        Instructions::Coshv(t) => coshv_values(val, *t, i).ok(),
        Instructions::Tanhv(t) => tanhv_values(val, *t, i).ok(),
        Instructions::Asinh(t) => asinh_values(val, *t, i).ok(),
        Instructions::Acosh(t) => acosh_values(val, *t, i).ok(),
        Instructions::Atanh(t) => atanh_values(val, *t, i).ok(),
        Instructions::Asinv(t) => asinv_values(val, *t, i).ok(),
        Instructions::Acosv(t) => acosv_values(val, *t, i).ok(),
        Instructions::Atanv(t) => atanv_values(val, *t, i).ok(),
        Instructions::Asinhv(t) => asinhv_values(val, *t, i).ok(),
        Instructions::Acoshv(t) => acoshv_values(val, *t, i).ok(),
        Instructions::Atanhv(t) => atanhv_values(val, *t, i).ok(),
        Instructions::Sqrt(t) => sqrt_values(val, *t, i).ok(),
        Instructions::Sqrtv(t) => sqrtv_values(val, *t, i).ok(),
        Instructions::Cbrt(t) => cbrt_values(val, *t, i).ok(),
        Instructions::Cbrtv(t) => cbrtv_values(val, *t, i).ok(),
        Instructions::Neg(t) => neg_values(val, *t, i).ok(),
        Instructions::Negv(t) => negv_values(val, *t, i).ok(),
        Instructions::Normalize(t) => normalize_values(val, *t, i).ok(),
        Instructions::Ln(t) => ln_values(val, *t, i).ok(),
        Instructions::Lnv(t) => lnv_values(val, *t, i).ok(),
        Instructions::Exp(t) => exp_values(val, *t, i).ok(),
        Instructions::Expv(t) => expv_values(val, *t, i).ok(),
        Instructions::Log2v(t) => log2v_values(val, *t, i).ok(),
        Instructions::Log10v(t) => log10v_values(val, *t, i).ok(),
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
  fn folds_valid_unary_float_vectors_and_retains_invalid_ones() {
    let operations = [
      Instructions::Lnv(PrimitiveTypes::Flt),
      Instructions::Log2v(PrimitiveTypes::Flt),
      Instructions::Log10v(PrimitiveTypes::Flt),
      Instructions::Sqrtv(PrimitiveTypes::Flt),
      Instructions::Cbrtv(PrimitiveTypes::Flt),
      Instructions::Expv(PrimitiveTypes::Flt),
    ];
    for operation in operations {
      let mut valid = vec![
        Instructions::PushArray(Arc::new(vec![Value::Float32(1.0)])),
        operation.clone(),
      ];
      fold_conversions(&mut valid);
      assert!(matches!(
        valid.as_slice(),
        [Instructions::PushArray(_), Instructions::Nop]
      ));
      let mut invalid = vec![
        Instructions::PushArray(Arc::new(vec![Value::Bool(false)])),
        operation.clone(),
      ];
      fold_conversions(&mut invalid);
      assert_eq!(invalid[1], operation);
    }
  }
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
  #[test]
  fn leaves_invalid_scalar_unary_operation_for_runtime_error() {
    let mut bytecode = vec![
      Instructions::PushString("invalid".into()),
      Instructions::Sin(PrimitiveTypes::Flt),
      Instructions::Stop,
    ];
    let expected = bytecode.clone();
    fold_conversions(&mut bytecode);
    assert_eq!(bytecode, expected);
    assert!(matches!(
      crate::vm::execute::execute(bytecode, &mut None, None),
      Err(crate::modules::vmerror::VMError::TypeMismatch {
        ip: 1,
        expected: "Float",
        found: "String"
      })
    ));
  }
  #[test]
  fn leaves_invalid_logarithm_operation_for_runtime_error() {
    let mut bytecode = vec![
      Instructions::PushString("invalid".into()),
      Instructions::Ln(PrimitiveTypes::Flt),
      Instructions::Stop,
    ];
    let expected = bytecode.clone();
    fold_conversions(&mut bytecode);
    assert_eq!(bytecode, expected);
    assert!(matches!(
      crate::vm::execute::execute(bytecode, &mut None, None),
      Err(crate::modules::vmerror::VMError::TypeMismatch {
        ip: 1,
        expected: "Float",
        found: "String"
      })
    ));
  }
}
