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
    add_func::add_values, div_func::div_values, mod_func::mod_values, mul_func::mul_values,
    pow_func::pow_values, powf_func::powf_values, powi_func::powi_values, rol_func::rol_values,
    ror_func::ror_values, shl_func::shl_values, shr_func::shr_values, sub_func::sub_values,
  },
  stack::concat_func::concat_values,
};
use crate::types::{instructions::Instructions, value::Value};
#[inline(always)]
fn extract_value(instr: &Instructions) -> Option<Value> {
  match instr {
    Instructions::PushInt16(v) => Some(Value::Int16(*v)),
    Instructions::PushInt32(v) => Some(Value::Int32(*v)),
    Instructions::PushInt64(v) => Some(Value::Int64(*v)),
    Instructions::PushInt128(v) => Some(Value::Int128(*v)),
    Instructions::PushFloat16(v) => Some(Value::Float16(*v)),
    Instructions::PushFloat32(v) => Some(Value::Float32(*v)),
    Instructions::PushFloat64(v) => Some(Value::Float64(*v)),
    Instructions::PushString(v) => Some(Value::String(v.clone())),
    Instructions::PushArray(v) => Some(Value::Array(v.clone())),
    Instructions::PushObject(v) => Some(Value::Object(v.clone())),
    Instructions::PushBool(v) => Some(Value::Bool(*v)),
    Instructions::PushNull => Some(Value::Null),
    Instructions::PushUndefined => Some(Value::Undefined),
    Instructions::PushNaN => Some(Value::NaN),
    Instructions::Push(v) => Some(v.clone()),
    _ => None,
  }
}
#[inline(always)]
fn value_to_instruction(val: Value) -> Instructions {
  match val {
    Value::Int16(v) => Instructions::PushInt16(v),
    Value::Int32(v) => Instructions::PushInt32(v),
    Value::Int64(v) => {
      if v >= i16::MIN as i64 && v <= i16::MAX as i64 {
        Instructions::PushInt16(v as i16)
      } else if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
        Instructions::PushInt32(v as i32)
      } else {
        Instructions::PushInt64(v)
      }
    }
    Value::Int128(v) => {
      if v >= i16::MIN as i128 && v <= i16::MAX as i128 {
        Instructions::PushInt16(v as i16)
      } else if v >= i32::MIN as i128 && v <= i32::MAX as i128 {
        Instructions::PushInt32(v as i32)
      } else if v >= i64::MIN as i128 && v <= i64::MAX as i128 {
        Instructions::PushInt64(v as i64)
      } else {
        Instructions::PushInt128(v)
      }
    }
    Value::Float16(v) => Instructions::PushFloat16(v),
    Value::Float32(v) => Instructions::PushFloat32(v),
    Value::Float64(v) => Instructions::PushFloat64(v),
    Value::String(v) => Instructions::PushString(v),
    Value::Array(v) => Instructions::PushArray(v),
    Value::Object(v) => Instructions::PushObject(v),
    Value::Bool(v) => Instructions::PushBool(v),
    Value::Null => Instructions::PushNull,
    Value::Undefined => Instructions::PushUndefined,
    Value::NaN => Instructions::PushNaN,
    other => Instructions::Push(other),
  }
}
#[inline(always)]
pub fn fold_constants(bytecode: &mut [Instructions]) {
  let mut i = 0;
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
        Instructions::Mul(t) => Some(mul_values(val1.clone(), val1.clone(), *t)),
        Instructions::Eq(t) => Some(eq_values(val1.clone(), val1.clone(), *t)),
        Instructions::Neq(t) => Some(neq_values(val1.clone(), val1.clone(), *t)),
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
