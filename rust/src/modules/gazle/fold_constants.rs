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
  comparison::{
    eq_func::eq_values, ge_func::ge_values, gt_func::gt_values, le_func::le_values,
    lt_func::lt_values, neq_func::neq_values,
  },
  logic::{and_func::and_values, or_func::or_values, xor_func::xor_values},
  math::{
    arithmetic::{
      add_func::add_values, div_func::div_values, mod_func::mod_values, mul_func::mul_values,
      pow_func::pow_values, powf_func::powf_values, powi_func::powi_values, sub_func::sub_values,
    },
    bitwise::{
      rotate::{rol_func::rol_values, ror_func::ror_values},
      shift::{shl_func::shl_values, shr_func::shr_values},
    },
    trigonometry::inverse::atan2_func::atan2_values,
    vector::{cross_func::cross_values, dot_func::dot_values},
  },
  stack::concat_func::concat_values,
};
use crate::modules::gazle::utils::{
  extract_value::extract_value, value_to_instruction::value_to_instruction,
};
use crate::types::{instructions::Instructions, value::Value};
use ahash::AHashMap;
use std::sync::Arc;
#[inline(always)]
pub fn fold_constants(bytecode: &mut [Instructions]) {
  let mut i = 0;
  while i < bytecode.len() {
    if let Some(Instructions::MakeArray(count)) = bytecode.get(i) {
      let count = *count as usize;
      if i >= count {
        let mut all_const = true;
        let mut vals = Vec::with_capacity(count);
        for instr in bytecode.iter().take(i).skip(i - count) {
          if let Some(val) = extract_value(instr) {
            vals.push(val);
          } else {
            all_const = false;
            break;
          }
        }
        if all_const {
          bytecode[i - count] = Instructions::PushArray(Arc::new(vals));
          for instr in bytecode.iter_mut().take(i + 1).skip(i - count + 1) {
            *instr = Instructions::Nop;
          }
          i += 1;
          continue;
        }
      }
    } else if let Some(Instructions::MakeObj(count)) = bytecode.get(i) {
      let operand_count = *count as usize * 2;
      if i >= operand_count {
        let mut operands = Vec::with_capacity(operand_count);
        for instr in &bytecode[(i - operand_count)..i] {
          if let Some(value) = extract_value(instr) {
            operands.push(value);
          } else {
            break;
          }
        }
        let valid_keys = operands
          .as_chunks::<2>()
          .0
          .iter()
          .all(|pair| matches!(&pair[0], Value::String(_)));
        if operands.len() == operand_count && valid_keys {
          let mut object = AHashMap::with_capacity(*count as usize);
          for pair in operands.as_chunks::<2>().0.iter().rev() {
            let Value::String(key) = &pair[0] else {
              unreachable!();
            };
            object.insert(key.clone(), pair[1].clone());
          }
          bytecode[i - operand_count] = Instructions::PushObject(Arc::new(object));
          for instr in bytecode.iter_mut().take(i + 1).skip(i - operand_count + 1) {
            *instr = Instructions::Nop;
          }
        }
      }
    }
    i += 1;
  }
  i = 0;
  while i < bytecode.len().saturating_sub(2) {
    let instr1 = &bytecode[i];
    let instr2 = &bytecode[i + 1];
    let instr3 = &bytecode[i + 2];
    if let (Some(val1), Some(val2)) = (extract_value(instr1), extract_value(instr2)) {
      let result = match instr3 {
        Instructions::Add(t) => Some(add_values(val1, val2, *t)),
        Instructions::Sub(t) => Some(sub_values(val1, val2, *t)),
        Instructions::Div(t) => Some(div_values(val1, val2, *t)),
        Instructions::Mul(t) => Some(mul_values(val1, val2, *t)),
        Instructions::Mod(t) => Some(mod_values(val1, val2, *t)),
        Instructions::Gt(t) => Some(gt_values(val1, val2, *t)),
        Instructions::Lt(t) => Some(lt_values(val1, val2, *t)),
        Instructions::Ge(t) => Some(ge_values(val1, val2, *t)),
        Instructions::Le(t) => Some(le_values(val1, val2, *t)),
        Instructions::Eq(t) => Some(eq_values(val1, val2, *t)),
        Instructions::Neq(t) => Some(neq_values(val1, val2, *t)),
        Instructions::Shl(t) => Some(shl_values(val1, val2, *t)),
        Instructions::Shr(t) => Some(shr_values(val1, val2, *t)),
        Instructions::Rol(t) => Some(rol_values(val1, val2, *t)),
        Instructions::Ror(t) => Some(ror_values(val1, val2, *t)),
        Instructions::And => Some(and_values(val1, val2)),
        Instructions::Or => Some(or_values(val1, val2)),
        Instructions::Xor => Some(xor_values(val1, val2)),
        Instructions::Concat => Some(concat_values(&val1, &val2)),
        Instructions::Pow(t) => Some(pow_values(val1, val2, *t)),
        Instructions::Powi(t) => Some(powi_values(val1, val2, *t)),
        Instructions::Powf(t) => Some(powf_values(val1, val2, *t)),
        Instructions::Atan2(t) => Some(atan2_values(val1, val2, *t)),
        Instructions::Dot(t) => Some(dot_values(val1, val2, *t)),
        Instructions::Cross(t) => Some(cross_values(val1, val2, *t)),
        _ => None,
      };
      if let Some(res_val) = result {
        bytecode[i] = value_to_instruction(res_val);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
        continue;
      }
    }
    if let Some(val1) = extract_value(instr1)
      && let Instructions::Dup = instr2
    {
      let result = match instr3 {
        Instructions::Add(t) => Some(add_values(val1.clone(), val1.clone(), *t)),
        Instructions::Sub(t) => Some(sub_values(val1.clone(), val1.clone(), *t)),
        Instructions::Div(t) => Some(div_values(val1.clone(), val1.clone(), *t)),
        Instructions::Mul(t) => Some(mul_values(val1.clone(), val1.clone(), *t)),
        Instructions::Mod(t) => Some(mod_values(val1.clone(), val1.clone(), *t)),
        Instructions::Gt(t) => Some(gt_values(val1.clone(), val1.clone(), *t)),
        Instructions::Lt(t) => Some(lt_values(val1.clone(), val1.clone(), *t)),
        Instructions::Ge(t) => Some(ge_values(val1.clone(), val1.clone(), *t)),
        Instructions::Le(t) => Some(le_values(val1.clone(), val1.clone(), *t)),
        Instructions::Eq(t) => Some(eq_values(val1.clone(), val1.clone(), *t)),
        Instructions::Neq(t) => Some(neq_values(val1.clone(), val1.clone(), *t)),
        Instructions::Shl(t) => Some(shl_values(val1.clone(), val1.clone(), *t)),
        Instructions::Shr(t) => Some(shr_values(val1.clone(), val1.clone(), *t)),
        Instructions::Rol(t) => Some(rol_values(val1.clone(), val1.clone(), *t)),
        Instructions::Ror(t) => Some(ror_values(val1.clone(), val1.clone(), *t)),
        Instructions::And => Some(and_values(val1.clone(), val1.clone())),
        Instructions::Or => Some(or_values(val1.clone(), val1.clone())),
        Instructions::Xor => Some(xor_values(val1.clone(), val1.clone())),
        Instructions::Concat => Some(concat_values(&val1, &val1)),
        Instructions::Pow(t) => Some(pow_values(val1.clone(), val1.clone(), *t)),
        Instructions::Powi(t) => Some(powi_values(val1.clone(), val1.clone(), *t)),
        Instructions::Powf(t) => Some(powf_values(val1.clone(), val1.clone(), *t)),
        Instructions::Atan2(t) => Some(atan2_values(val1.clone(), val1.clone(), *t)),
        Instructions::Cross(t) => Some(cross_values(val1.clone(), val1.clone(), *t)),
        _ => None,
      };
      if let Some(res_val) = result {
        bytecode[i] = value_to_instruction(res_val);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
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
  use smol_str::SmolStr;
  #[test]
  fn folds_constant_make_array() {
    let mut bytecode = vec![
      Instructions::PushInt16(1),
      Instructions::PushInt16(2),
      Instructions::MakeArray(2),
    ];
    fold_constants(&mut bytecode);
    assert_eq!(
      bytecode,
      vec![
        Instructions::PushArray(Arc::new(vec![Value::Int16(1), Value::Int16(2)])),
        Instructions::Nop,
        Instructions::Nop,
      ]
    );
  }
  #[test]
  fn leaves_non_constant_make_array_unchanged() {
    let mut bytecode = vec![
      Instructions::Get(SmolStr::new("value")),
      Instructions::MakeArray(1),
    ];
    let expected = bytecode.clone();
    fold_constants(&mut bytecode);
    assert_eq!(bytecode, expected);
  }
  #[test]
  fn folds_arithmetic_with_make_array_present() {
    let mut bytecode = vec![
      Instructions::PushInt16(1),
      Instructions::MakeArray(1),
      Instructions::PushInt16(2),
      Instructions::PushInt16(3),
      Instructions::Add(PrimitiveTypes::Sht),
    ];
    fold_constants(&mut bytecode);
    assert_eq!(
      bytecode,
      vec![
        Instructions::PushArray(Arc::new(vec![Value::Int16(1)])),
        Instructions::Nop,
        Instructions::PushInt16(5),
        Instructions::Nop,
        Instructions::Nop,
      ]
    );
  }
  #[test]
  fn folds_constant_make_obj() {
    let mut bytecode = vec![
      Instructions::PushString(SmolStr::new("first")),
      Instructions::PushInt16(1),
      Instructions::PushString(SmolStr::new("second")),
      Instructions::PushInt16(2),
      Instructions::MakeObj(2),
    ];
    let mut expected_object = AHashMap::new();
    expected_object.insert(SmolStr::new("first"), Value::Int16(1));
    expected_object.insert(SmolStr::new("second"), Value::Int16(2));
    fold_constants(&mut bytecode);
    assert_eq!(
      bytecode,
      vec![
        Instructions::PushObject(Arc::new(expected_object)),
        Instructions::Nop,
        Instructions::Nop,
        Instructions::Nop,
        Instructions::Nop,
      ]
    );
  }
  #[test]
  fn leaves_non_constant_make_obj_unchanged() {
    let bytecodes = [
      vec![
        Instructions::Get(SmolStr::new("key")),
        Instructions::PushInt16(1),
        Instructions::MakeObj(1),
      ],
      vec![
        Instructions::PushString(SmolStr::new("key")),
        Instructions::Get(SmolStr::new("value")),
        Instructions::MakeObj(1),
      ],
    ];
    for mut bytecode in bytecodes {
      let expected = bytecode.clone();
      fold_constants(&mut bytecode);
      assert_eq!(bytecode, expected);
    }
  }
  #[test]
  fn leaves_non_string_make_obj_key_unchanged() {
    let mut bytecode = vec![
      Instructions::PushInt16(1),
      Instructions::PushInt16(2),
      Instructions::MakeObj(1),
    ];
    let expected = bytecode.clone();
    fold_constants(&mut bytecode);
    assert_eq!(bytecode, expected);
  }
}
