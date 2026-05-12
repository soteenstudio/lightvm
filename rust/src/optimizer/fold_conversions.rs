/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{instructions::Instructions, value::Value};
#[inline]
#[cold]
pub fn fold_conversions(bytecode: &mut [Instructions]) {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(1) {
    let mut instr1 = std::mem::replace(&mut bytecode[i], Instructions::Nop);
    let mut instr2 = std::mem::replace(&mut bytecode[i + 1], Instructions::Nop);
    match (&mut instr1, &mut instr2) {
      (Instructions::Push(v), Instructions::Not) => {
        let res = crate::instructions::logic::not_func::not_func(std::mem::replace(v, Value::Null));
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(v), Instructions::ToShort) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_short_func::to_short_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToInteger) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_integer_func::to_integer_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToLong) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_long_func::to_long_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToHalf) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_half_func::to_half_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToFloat) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_float_func::to_float_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToDouble) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_double_func::to_double_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::ToString) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::conversion::to_string_func::to_string_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::TypeOf) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::metadata::typeof_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::Sin) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::math::sin_func::sin_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::Cos) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::math::cos_func::cos_func(&mut tmp_stack);
        if let Some(converted_val) = tmp_stack.pop() {
          bytecode[i] = Instructions::Push(converted_val);
          bytecode[i + 1] = Instructions::Nop;
          i += 2;
        } else {
          bytecode[i] = Instructions::Push(std::mem::replace(v, Value::Null));
          i += 1;
        }
      }
      (Instructions::Push(v), Instructions::Tan) => {
        let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
        crate::instructions::math::tan_func::tan_func(&mut tmp_stack);
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
