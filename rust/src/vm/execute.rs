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
  collections::{
    access_func::access_func, access_index_func::access_index_func,
    make_array_func::make_array_func, make_obj_func::make_obj_func,
  },
  comparison::{
    eq_func::eq_func, ge_func::ge_func, gt_func::gt_func, le_func::le_func, lt_func::lt_func,
    neq_func::neq_func,
  },
  control_flow::{
    call_func::call_func, if_false_func::if_false_func, jump_func::jump_func,
    return_func::return_func, stop_func::stop_func,
  },
  conversion::{
    to_double_func::to_double_func, to_float_func::to_float_func, to_integer_func::to_integer_func,
    to_long_func::to_long_func, to_string_func::to_string_func,
  },
  io::{
    inspect_arr_func::inspect_arr_func, inspect_obj_func::inspect_obj_func, print_func::print_func,
    println_func::println_func,
  },
  logic::{and_func::and_func, or_func::or_func},
  math::{
    add_func::add_func,
    div_func::div_func,
    inc_dec::{dec_func, inc_func},
    mod_func::mod_func,
    mul_func::mul_func,
    sub_func::sub_func,
  },
  metadata::{length_func::length_func, typeof_func::typeof_func},
  stack::{
    concat_func::concat_func, dup_func::dup_func, get_func::get_func, push_func::push_func,
    set_func::set_func, val_func::val_func,
  },
};
use crate::types::{
  instructions::Instructions,
  value::{RunOptions, Value},
};
use crate::utils::{
  compute_hot_threshold::compute_hot_threshold, resolve_symbols::resolve_symbols,
};
use crate::vm::{inject_args::inject_args, prepare_vm::prepare_vm};
use smol_str::SmolStr;
pub fn execute(
  mut bytecode: Vec<Instructions>,
  options: Option<RunOptions>,
) -> Result<Value, SmolStr> {
  let mut last_return = Value::Undefined;
  let mut stack: Vec<Value> = Vec::with_capacity(100);
  let var_count = resolve_symbols(&mut bytecode);
  let mut vars: Vec<Value> = vec![Value::Undefined; var_count];
  let mut _call_stack: Vec<usize> = Vec::new();
  let mut hit_counter = vec![0u32; bytecode.len()];
  let (functions, _exported, mut ip) = prepare_vm(&bytecode, &options);
  inject_args(&mut vars, &functions, &options, ip);
  while ip < bytecode.len() {
    let instr = &bytecode[ip];
    hit_counter[ip] += 1;
    let is_hot = hit_counter[ip] >= 1000;
    let _hot_threshold = compute_hot_threshold(stack.len());
    match instr {
      Instructions::Push(val) => {
        push_func(&mut stack, val.clone());
      }
      Instructions::ValIdx(idx) => {
        val_func(&mut vars, *idx);
      }
      Instructions::SetIdx(idx) => {
        set_func(&mut stack, &mut vars, *idx);
      }
      Instructions::GetIdx(idx) => {
        get_func(&mut stack, &mut vars, *idx);
      }
      Instructions::Concat => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on CONCAT (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on CONCAT (a)"))?;
        let result = concat_func(&a, &b);
        stack.push(result);
      }
      Instructions::Add(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on ADD (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on ADD (a)"))?;
        let result = add_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Sub(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on SUB (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on SUB (a)"))?;
        let result = sub_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Mul(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on MUL (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on MUL (a)"))?;
        let result = mul_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Div(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on DIV (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on DIV (a)"))?;
        let result = div_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Mod(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on MOD (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on MOD (a)"))?;
        let result = mod_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Gt(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on GT (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on GT (a)"))?;
        let result = gt_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Lt(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on LT (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on LT (a)"))?;
        let result = lt_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Ge(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on GE (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on GE (a)"))?;
        let result = ge_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Le(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on LE (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on LE (a)"))?;
        let result = le_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Eq(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on EQ (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on EQ (a)"))?;
        let result = eq_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Neq(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on NEQ (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on NEQ (a)"))?;
        let result = neq_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::And => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on AND (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on AND (a)"))?;
        let result = and_func(a, b);
        stack.push(result);
      }
      Instructions::Or => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on OR (b)"))?;
        let a = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on OR (a)"))?;
        let result = or_func(a, b);
        stack.push(result);
      }
      Instructions::Print => {
        if let Some(val) = stack.pop() {
          print_func(val);
        } else {
          panic!("Stack underflow on PRINT");
        }
      }
      Instructions::Println => {
        if let Some(val) = stack.pop() {
          println_func(val);
        } else {
          panic!("Stack underflow on PRINTLN");
        }
      }
      Instructions::IfFalse(target_ip) => {
        let cond = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on IF_FALSE"))?;
        if if_false_func(cond) {
          ip = *target_ip;
          continue;
        }
      }
      Instructions::Jump(target_ip) => {
        jump_func(&mut ip, *target_ip, &mut stack);
        continue;
      }
      Instructions::Return => {
        if return_func(&mut stack, &mut _call_stack, &mut ip, &mut last_return) {
          continue;
        } else {
          break;
        }
      }
      Instructions::Call(name, argc) => {
        call_func(
          name,
          *argc,
          &mut ip,
          &mut stack,
          &mut _call_stack,
          &mut vars,
          &functions,
        )?;
        continue;
      }
      Instructions::Stop => {
        if stop_func(&mut stack, &mut _call_stack, &mut ip) {
          continue;
        } else {
          break;
        }
      }
      Instructions::IncIdx(idx, num_type) => {
        inc_func(&mut vars, &mut stack, *idx, *num_type, is_hot)?;
      }
      Instructions::DecIdx(idx, num_type) => {
        dec_func(&mut vars, *idx, *num_type)?;
      }
      Instructions::MakeObj(count) => {
        make_obj_func(&mut stack, *count)?;
      }
      Instructions::MakeArray(count) => {
        make_array_func(&mut stack, *count)?;
      }
      Instructions::AccessIndex => {
        access_index_func(&mut stack)?;
      }
      Instructions::Access(prop) => {
        access_func(&mut stack, prop)?;
      }
      Instructions::TypeOf => {
        typeof_func(&mut stack)?;
      }
      Instructions::InspectObj => {
        inspect_obj_func(&mut stack)?;
      }
      Instructions::InspectArr => {
        inspect_arr_func(&mut stack)?;
      }
      Instructions::ToString => {
        to_string_func(&mut stack);
      }
      Instructions::ToInteger => {
        to_integer_func(&mut stack);
      }
      Instructions::ToLong => {
        to_long_func(&mut stack);
      }
      Instructions::ToFloat => {
        to_float_func(&mut stack);
      }
      Instructions::ToDouble => {
        to_double_func(&mut stack);
      }
      Instructions::Dup => {
        dup_func(&mut stack);
      }
      Instructions::Length => {
        length_func(&mut stack);
      }
      _ => {}
    }
    ip += 1;
  }
  if options.as_ref().is_some_and(|o| o.capture_return) {
    return Ok(last_return);
  }
  Ok(Value::Undefined)
}
