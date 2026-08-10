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
  math::{cos_func::cos_values, neg_func::neg_values, sin_func::sin_values, tan_func::tan_values},
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
        Instructions::Neg(t) => Some(neg_values(val, *t)),
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
