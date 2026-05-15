/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::{
  comparison::{
    eq_func::eq_values, ge_func::ge_values, gt_func::gt_values, le_func::le_values,
    lt_func::lt_values, neq_func::neq_values,
  },
  math::{
    add_func::add_values, div_func::div_values, mod_func::mod_values, mul_func::mul_values,
    pow_func::pow_values, powf_func::powf_values, powi_func::powi_values, rol_func::rol_values,
    ror_func::ror_values, shl_func::shl_values, shr_func::shr_values, sub_func::sub_values,
  },
  stack::concat_func::concat_values,
};
use crate::types::{instructions::Instructions, value::Value};
#[inline]
#[cold]
pub fn fold_constants(bytecode: &mut [Instructions]) {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(2) {
    let mut instr1 = std::mem::replace(&mut bytecode[i], Instructions::Nop);
    let mut instr2 = std::mem::replace(&mut bytecode[i + 1], Instructions::Nop);
    let mut instr3 = std::mem::replace(&mut bytecode[i + 2], Instructions::Nop);
    match (&mut instr1, &mut instr2, &mut instr3) {
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Add(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = add_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Sub(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = sub_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Div(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = div_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mul(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = mul_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mod(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = mod_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Gt(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = gt_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Lt(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = lt_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Ge(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = ge_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Le(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = le_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Eq(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = eq_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Neq(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = neq_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Shl(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let res = shl_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Shr(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let res = shr_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Rol(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let res = rol_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Ror(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let res = ror_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::And) => {
        let res = crate::instructions::logic::and_func::and_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Or) => {
        let res = crate::instructions::logic::or_func::or_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Xor) => {
        let res = crate::instructions::logic::xor_func::xor_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Concat) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let res = concat_values(&val1, &val2);
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Pow(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = pow_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Powi(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = powi_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Powf(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = powf_values(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      _ => {
        bytecode[i] = instr1;
        bytecode[i + 1] = instr2;
        bytecode[i + 2] = instr3;
        i += 1;
      }
    }
  }
}
