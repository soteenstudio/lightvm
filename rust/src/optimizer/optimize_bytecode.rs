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
  add_func::add_func, div_func::div_func, mod_func::mod_func, mul_func::mul_func,
  sub_func::sub_func,
};
use crate::optimizer::{
  analyze_usage::analyze_usage, eliminate_dead_loops::eliminate_dead_loops,
  eliminate_dead_stores::eliminate_dead_stores,
};
use crate::types::{instructions::Instructions, value::Value};
use std::borrow::Cow;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(2) {
    let mut instr1 = std::mem::replace(&mut bytecode[i], Instructions::Nop);
    let mut instr2 = std::mem::replace(&mut bytecode[i + 1], Instructions::Nop);
    let mut instr3 = std::mem::replace(&mut bytecode[i + 2], Instructions::Nop);
    match (&mut instr1, &mut instr2, &mut instr3) {
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Add(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = add_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Sub(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = sub_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Div(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = div_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mul(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = mul_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Mod(t)) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let result = mod_func(val1, val2, *t);
        bytecode[i] = Instructions::Push(result);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Gt(t)) => {
        let res = crate::instructions::comparison::gt_func::gt_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Lt(t)) => {
        let res = crate::instructions::comparison::lt_func::lt_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Ge(t)) => {
        let res = crate::instructions::comparison::ge_func::ge_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Le(t)) => {
        let res = crate::instructions::comparison::le_func::le_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Eq(t)) => {
        let res = crate::instructions::comparison::eq_func::eq_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Neq(t)) => {
        let res = crate::instructions::comparison::neq_func::neq_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Shl(t)) => {
        let res = crate::instructions::math::shl_func::shl_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Shr(t)) => {
        let res = crate::instructions::math::shr_func::shr_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Rol(t)) => {
        let res = crate::instructions::math::rol_func::rol_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
        bytecode[i] = Instructions::Push(res);
        bytecode[i + 1] = Instructions::Nop;
        bytecode[i + 2] = Instructions::Nop;
        i += 3;
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Ror(t)) => {
        let res = crate::instructions::math::ror_func::ror_func(
          std::mem::replace(v1, Value::Null),
          std::mem::replace(v2, Value::Null),
          *t,
        );
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
      _ if i < bytecode.len().saturating_sub(1) => {
        let mut instr1 = std::mem::replace(&mut bytecode[i], Instructions::Nop);
        let mut instr2 = std::mem::replace(&mut bytecode[i + 1], Instructions::Nop);
        match (&mut instr1, &mut instr2) {
          (Instructions::Push(v), Instructions::Not) => {
            let res =
              crate::instructions::logic::not_func::not_func(std::mem::replace(v, Value::Null));
            bytecode[i] = Instructions::Push(res);
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToShort) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_short_func::to_short_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToInteger) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_integer_func::to_integer_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToLong) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_long_func::to_long_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToHalf) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_half_func::to_half_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToFloat) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_float_func::to_float_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToDouble) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_double_func::to_double_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::ToString) => {
            let mut tmp_stack = vec![std::mem::replace(v, Value::Null)];
            crate::instructions::conversion::to_string_func::to_string_func(&mut tmp_stack);
            bytecode[i] = Instructions::Push(tmp_stack.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          (Instructions::Push(v), Instructions::TypeOf) => {
            let mut tmp = vec![std::mem::replace(v, Value::Null)];
            let _ = crate::instructions::metadata::typeof_func::typeof_func(&mut tmp);
            bytecode[i] = Instructions::Push(tmp.pop().unwrap());
            bytecode[i + 1] = Instructions::Nop;
            i += 2;
          }
          _ => {
            bytecode[i] = instr1;
            bytecode[i + 1] = instr2;
            i += 1;
          }
        }
      }
      (Instructions::Push(v1), Instructions::Push(v2), Instructions::Concat) => {
        let val1 = std::mem::replace(v1, Value::Null);
        let val2 = std::mem::replace(v2, Value::Null);
        let res = crate::instructions::stack::concat_func::concat_func(&val1, &val2);
        bytecode[i] = Instructions::Push(res);
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
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
  let usage = analyze_usage(&bytecode);
  bytecode = eliminate_dead_stores(&bytecode, &usage)
    .into_iter()
    .map(Cow::into_owned)
    .collect();
  bytecode = eliminate_dead_loops(bytecode);
  let mut j = 0;
  while j < bytecode.len() {
    if let Instructions::Jump(target) = bytecode[j] {
      if target == j + 1 {
        bytecode[j] = Instructions::Nop;
      }
    }
    j += 1;
  }
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
  bytecode
}
