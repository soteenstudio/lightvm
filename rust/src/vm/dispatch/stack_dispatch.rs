/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::stack::{
  concat_func::concat_func, dup_func::dup_func, get_func::get_func,
  push_array_func::push_array_func, push_f16_func::push_f16_func, push_f32_func::push_f32_func,
  push_f64_func::push_f64_func, push_func::push_func, push_i16_func::push_i16_func,
  push_i32_func::push_i32_func, push_i64_func::push_i64_func, push_i128_func::push_i128_func,
  push_object_func::push_object_func, push_string_func::push_string_func, set_func::set_func,
  swap_func::swap_func, truncate_func::truncate_func, val_func::val_func,
};
use crate::types::stack::Stack;
use crate::types::{instructions::Instructions, value::Value, var_stack::VarStack};
use crate::utils::vmerror::VMError;
const MAX_STACK_RESERVATION: usize = 65_536;
#[inline(always)]
pub fn stack_dispatch(
  instr: &Instructions,
  stack: &mut Stack,
  vars: &mut VarStack,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::InitStack(size) => {
      let reserve_size = *size as usize;
      if ip == 0 {
        if reserve_size > MAX_STACK_RESERVATION {
          return Err(VMError::StackOverflow {
            ip,
            limit: MAX_STACK_RESERVATION,
          });
        }
        stack.clear();
        stack.reserve(*size as usize);
      }
      Ok(())
    }
    Instructions::PushInt16(v) => push_i16_func(stack, v, ip),
    Instructions::PushInt32(v) => push_i32_func(stack, v, ip),
    Instructions::PushInt64(v) => push_i64_func(stack, v, ip),
    Instructions::PushInt128(v) => push_i128_func(stack, v, ip),
    Instructions::PushFloat16(v) => push_f16_func(stack, v, ip),
    Instructions::PushFloat32(v) => push_f32_func(stack, v, ip),
    Instructions::PushFloat64(v) => push_f64_func(stack, v, ip),
    Instructions::PushString(v) => push_string_func(stack, v, ip),
    Instructions::PushArray(v) => push_array_func(stack, v, ip),
    Instructions::PushObject(v) => push_object_func(stack, v, ip),
    Instructions::PushUndefined => {
      stack.push(Value::Undefined);
      Ok(())
    }
    Instructions::PushNull => {
      stack.push(Value::Null);
      Ok(())
    }
    Instructions::PushNaN => {
      stack.push(Value::NaN);
      Ok(())
    }
    Instructions::Push(val) => push_func(stack, val.clone(), ip),
    Instructions::ValIdx(idx) => val_func(vars, *idx, ip),
    Instructions::SetIdx(idx) => set_func(stack, vars, *idx, ip),
    Instructions::GetIdx(idx) => get_func(stack, vars, *idx, ip),
    Instructions::Concat => concat_func(stack, ip),
    Instructions::Dup => dup_func(stack, ip),
    Instructions::Swap => swap_func(stack, ip),
    Instructions::Truncate => truncate_func(stack, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
