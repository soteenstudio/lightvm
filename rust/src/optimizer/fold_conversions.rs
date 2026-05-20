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
  conversion::{
    to_double_func::to_double_values, to_float_func::to_float_values, to_half_func::to_half_values,
    to_integer_func::to_integer_values, to_long_func::to_long_values, to_octa_func::to_octa_values,
    to_short_func::to_short_values, to_string_func::to_string_values,
  },
  logic::not_func::not_values,
  math::{cos_func::cos_values, neg_func::neg_values, sin_func::sin_values, tan_func::tan_values},
  metadata::typeof_func::typeof_values,
};
use crate::types::{instructions::Instructions, value::Value};
#[inline(always)]
pub fn fold_conversions(bytecode: &mut [Instructions]) {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(1) {
    let mut instr1 = std::mem::replace(&mut bytecode[i], Instructions::Nop);
    let mut instr2 = std::mem::replace(&mut bytecode[i + 1], Instructions::Nop);
    match (&mut instr1, &mut instr2) {
      (Instructions::Push(v), Instructions::Not) => {
        let val = std::mem::replace(v, Value::Null);
        let res = not_values(val);
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(v), Instructions::ToShort) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_short_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToInteger) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_integer_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToLong) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_long_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToOcta) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_octa_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToHalf) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_half_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToFloat) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_float_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToDouble) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_double_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToString) => {
        let owned_val = std::mem::replace(v, Value::Null);
        if let Ok(converted_val) = to_string_values(owned_val) {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::TypeOf) => {
        let owned_val = std::mem::replace(v, Value::Null);
        let converted_val = typeof_values(owned_val);
        bytecode[i] = Instructions::Push(converted_val);
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(v), Instructions::Sin(t)) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        let res = sin_values(tmp_stack.pop().unwrap(), *t);
        tmp_stack.push(res);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::Cos(t)) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        let res = cos_values(tmp_stack.pop().unwrap(), *t);
        tmp_stack.push(res);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::Tan(t)) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        let res = tan_values(tmp_stack.pop().unwrap(), *t);
        tmp_stack.push(res);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::Neg(t)) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        let res = neg_values(tmp_stack.pop().unwrap(), *t);
        tmp_stack.push(res);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      _ => {
        bytecode[i] = instr1;
        bytecode[i + 1] = instr2;
        i += 1;
      }
    }
  }
}
