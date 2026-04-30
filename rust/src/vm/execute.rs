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
    eq_func::eq_func, ge_func::ge_func, gt_func::gt_func, le_func::le_func, lt_func::lt_func,
  },
  control_flow::{
    call_func::call_func, if_false_func::if_false_func, jump_func::jump_func,
    return_func::return_func, stop_func::stop_func,
  },
  io::{print::print_func, println::println_func},
  math::{
    add_func::add_func,
    div_func::div_func,
    inc_dec::{dec_func, inc_func},
    mod_func::mod_func,
    mul_func::mul_func,
    sub_func::sub_func,
  },
  stack::{push_func::push_func, val_func::val_func},
};
use crate::types::{
  instructions::Instructions,
  value::{FuncMetadata, RunOptions, Value},
};
use crate::utils::{
  compute_hot_threshold::compute_hot_threshold, resolve_symbols::resolve_symbols,
};
use std::collections::{HashMap, HashSet};
pub fn execute(
  mut bytecode: Vec<Instructions>,
  options: Option<RunOptions>,
) -> Result<Value, String> {
  let mut last_return = Value::Undefined;
  let mut stack: Vec<Value> = Vec::with_capacity(100);
  let var_count = resolve_symbols(&mut bytecode);
  let mut vars: Vec<Value> = vec![Value::Undefined; var_count];
  let mut _call_stack: Vec<usize> = Vec::new();
  let mut hit_counter = vec![0u32; bytecode.len()];
  let mut ip: usize = options.as_ref().and_then(|o| o.entry).unwrap_or(0);
  let mut functions: HashMap<String, FuncMetadata> = HashMap::new();
  let mut exported: HashSet<String> = HashSet::new();
  for instr in bytecode.iter() {
    if let Instructions::Func(name, params, start, end, names) = instr {
      functions.insert(
        name.clone(),
        FuncMetadata {
          params_count: *params,
          param_names: names.clone(),
          start: *start,
          end: *end,
        },
      );
    }
    if let Instructions::Export(name) = instr {
      exported.insert(name.clone());
    }
  }
  if let Some(opts) = &options {
    if let Some(entry_ip) = opts.entry {
      let entry_fn = functions.values().find(|f| f.start == entry_ip);
      if let Some(fn_meta) = entry_fn {
        for i in 0..fn_meta.params_count as usize {
          let val = opts.args.get(i).cloned().unwrap_or(Value::Undefined);
          if i < vars.len() {
            vars[i] = val;
          }
        }
      }
    }
  }
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
        if let Some(val) = stack.pop() {
          vars[*idx] = val;
        }
      }
      Instructions::GetIdx(idx) => {
        stack.push(vars[*idx].clone());
      }
      Instructions::Add(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on ADD (b)")?;
        let a = stack.pop().ok_or("Stack underflow on ADD (a)")?;
        let result = add_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Sub(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on SUB (b)")?;
        let a = stack.pop().ok_or("Stack underflow on SUB (a)")?;
        let result = sub_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Mul(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on MUL (b)")?;
        let a = stack.pop().ok_or("Stack underflow on MUL (a)")?;
        let result = mul_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Div(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on DIV (b)")?;
        let a = stack.pop().ok_or("Stack underflow on DIV (a)")?;
        let result = div_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Mod(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on MOD (b)")?;
        let a = stack.pop().ok_or("Stack underflow on MOD (a)")?;
        let result = mod_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Gt(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on GT (b)")?;
        let a = stack.pop().ok_or("Stack underflow on GT (a)")?;
        let result = gt_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Lt(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on LT (b)")?;
        let a = stack.pop().ok_or("Stack underflow on LT (a)")?;
        let result = lt_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Ge(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on GE (b)")?;
        let a = stack.pop().ok_or("Stack underflow on GE (a)")?;
        let result = ge_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Le(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on LE (b)")?;
        let a = stack.pop().ok_or("Stack underflow on LE (a)")?;
        let result = le_func(a, b, *num_type);
        stack.push(result);
      }
      Instructions::Eq(num_type) => {
        let b = stack.pop().ok_or("Stack underflow on EQ (b)")?;
        let a = stack.pop().ok_or("Stack underflow on EQ (a)")?;
        let result = eq_func(a, b, *num_type);
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
        let cond = stack.pop().ok_or("Stack underflow on IF_FALSE")?;
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
      _ => {}
    }
    ip += 1;
  }
  if options.as_ref().is_some_and(|o| o.capture_return) {
    return Ok(last_return);
  }
  Ok(Value::Undefined)
}
