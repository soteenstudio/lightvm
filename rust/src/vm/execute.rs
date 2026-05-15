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
    make_array_func::make_array_func, make_obj_func::make_obj_func, set_prop_func::set_prop_func,
    shrink_func::shrink_func,
  },
  comparison::{
    eq_func::eq_func, ge_func::ge_func, gt_func::gt_func, le_func::le_func, lt_func::lt_func,
    neq_func::neq_func,
  },
  control_flow::{
    break_func::break_func, call_func::call_func, if_false_func::if_false_func,
    instantiate_func::instantiate_func, jump_func::jump_func, return_func::return_func,
    stop_func::stop_func,
  },
  conversion::{
    to_double_func::to_double_func, to_float_func::to_float_func, to_half_func::to_half_func,
    to_integer_func::to_integer_func, to_long_func::to_long_func, to_octa_func::to_octa_func,
    to_short_func::to_short_func, to_string_func::to_string_func,
  },
  io::{
    inspect_arr_func::inspect_arr_func, inspect_obj_func::inspect_obj_func, print_func::print_func,
    println_func::println_func,
  },
  logic::{and_func::and_func, not_func::not_func, or_func::or_func, xor_func::xor_func},
  math::{
    add_func::add_func,
    cos_func::cos_func,
    div_func::div_func,
    inc_dec::{dec_func, inc_func},
    mul_func::mul_func,
    pow_func::pow_func,
    powf_func::powf_func,
    powi_func::powi_func,
    rol_func::rol_func,
    ror_func::ror_func,
    shl_func::shl_func,
    shr_func::shr_func,
    sin_func::sin_func,
    sub_func::sub_func,
    tan_func::tan_func,
  },
  metadata::{length_func::length_func, typeof_func::typeof_func},
  stack::{
    concat_func::concat_func, dup_func::dup_func, get_func::get_func, import_func::import_func,
    push_f16_func::push_f16_func, push_f32_func::push_f32_func, push_f64_func::push_f64_func,
    push_func::push_func, push_i128_func::push_i128_func, push_i16_func::push_i16_func,
    push_i32_func::push_i32_func, push_i64_func::push_i64_func, set_func::set_func,
    truncate_func::truncate_func, val_func::val_func,
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
#[inline(never)]
pub fn execute(
  mut bytecode: Vec<Instructions>,
  options: Option<RunOptions>,
) -> Result<Value, SmolStr> {
  let mut last_return = Value::Undefined;
  let mut stack: Vec<Value> = Vec::with_capacity(16);
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
      Instructions::InitStack(size) => {
        if ip == 0 {
          stack = Vec::with_capacity(*size as usize);
        }
      }
      Instructions::PushInt16(v) => {
        push_i16_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushInt32(v) => {
        push_i32_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushInt64(v) => {
        push_i64_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushInt128(v) => {
        push_i128_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushFloat16(v) => {
        push_f16_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushFloat32(v) => {
        push_f32_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushFloat64(v) => {
        push_f64_func(&mut stack, v, ip).map_err(|e| e.format())?;
      }
      Instructions::PushUndefined => {
        stack.push(Value::Undefined);
      }
      Instructions::Push(val) => {
        push_func(&mut stack, val.clone());
      }
      Instructions::ValIdx(idx) => {
        val_func(&mut vars, *idx);
      }
      Instructions::SetIdx(idx) => {
        set_func(&mut stack, &mut vars, *idx, ip).map_err(|e| e.format())?;
      }
      Instructions::GetIdx(idx) => {
        get_func(&mut stack, &mut vars, *idx, ip).map_err(|e| e.format())?;
      }
      Instructions::Concat => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on CONCAT (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on CONCAT (a)"))?;
        *a_ref = concat_func(a_ref, &b);
      }
      Instructions::Add(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on ADD (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on ADD (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = add_func(a, b, *num_type);
      }
      Instructions::Sub(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on SUB (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on SUB (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = sub_func(a, b, *num_type);
      }
      Instructions::Mul(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on MUL (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on MUL (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = mul_func(a, b, *num_type);
      }
      Instructions::Div(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on DIV (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on DIV (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = div_func(a, b, *num_type);
      }
      Instructions::Mod(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on MOD (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on MOD (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = div_func(a, b, *num_type);
      }
      Instructions::Shl(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on SHL (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on SHL (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = shl_func(a, b, *num_type);
      }
      Instructions::Shr(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on SHR (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on SHR (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = shr_func(a, b, *num_type);
      }
      Instructions::Ror(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on ROR (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on ROR (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = ror_func(a, b, *num_type);
      }
      Instructions::Rol(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on ROL (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on ROL (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = rol_func(a, b, *num_type);
      }
      Instructions::Pow(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on POW (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on POW (a)"))?;
        println!("Pow jalan!");
        let a = std::mem::take(a_ref);
        *a_ref = pow_func(a, b, *num_type);
      }
      Instructions::Powi(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on POWI (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on POWI (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = powi_func(a, b, *num_type);
      }
      Instructions::Powf(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on POWF (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on POWF (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = powf_func(a, b, *num_type);
      }
      Instructions::Sin(num_type) => {
        let val_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on SIN"))?;
        let val = std::mem::take(val_ref);
        *val_ref = sin_func(val, *num_type);
      }
      Instructions::Cos(num_type) => {
        let val_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on COS"))?;
        let val = std::mem::take(val_ref);
        *val_ref = cos_func(val, *num_type);
      }
      Instructions::Tan(num_type) => {
        let val_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on TAN"))?;
        let val = std::mem::take(val_ref);
        *val_ref = tan_func(val, *num_type);
      }
      Instructions::Gt(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on GT (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on GT (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = gt_func(a, b, *num_type);
      }
      Instructions::Lt(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on LT (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on LT (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = lt_func(a, b, *num_type);
      }
      Instructions::Ge(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on GE (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on GE (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = ge_func(a, b, *num_type);
      }
      Instructions::Le(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on LE (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on LE (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = le_func(a, b, *num_type);
      }
      Instructions::Eq(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on EQ (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on EQ (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = eq_func(a, b, *num_type);
      }
      Instructions::Neq(num_type) => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on NEQ (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on NEQ (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = neq_func(a, b, *num_type);
      }
      Instructions::And => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on AND (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on AND (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = and_func(a, b);
      }
      Instructions::Or => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on OR (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on OR (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = or_func(a, b);
      }
      Instructions::Xor => {
        let b = stack
          .pop()
          .ok_or_else(|| SmolStr::new("Stack underflow on XOR (b)"))?;
        let a_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on XOR (a)"))?;
        let a = std::mem::take(a_ref);
        *a_ref = xor_func(a, b);
      }
      Instructions::Not => {
        let val_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on NOT"))?;
        let val = std::mem::take(val_ref);
        *val_ref = not_func(val);
      }
      Instructions::Print => {
        let val_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on PRINT"))?;
        let val = std::mem::take(val_ref);
        print_func(val);
      }
      Instructions::Println => {
        let val_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on PRINTLN"))?;
        let val = std::mem::take(val_ref);
        println_func(val);
      }
      Instructions::IfFalse(target_ip) => {
        let cond_ref = stack
          .last_mut()
          .ok_or_else(|| SmolStr::new("Stack underflow on IF_FALSE"))?;
        let cond = std::mem::take(cond_ref);
        if if_false_func(cond) {
          ip = *target_ip;
          continue;
        }
      }
      Instructions::Jump(target_ip) => {
        jump_func(&mut ip, *target_ip);
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
        if stop_func(&mut _call_stack, &mut ip) {
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
        typeof_func(&mut stack, ip).map_err(|e| e.format())?;
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
      Instructions::ToShort => {
        to_short_func(&mut stack);
      }
      Instructions::ToInteger => {
        to_integer_func(&mut stack);
      }
      Instructions::ToLong => {
        to_long_func(&mut stack);
      }
      Instructions::ToOcta => {
        to_octa_func(&mut stack);
      }
      Instructions::ToHalf => {
        to_half_func(&mut stack);
      }
      Instructions::ToFloat => {
        to_float_func(&mut stack);
      }
      Instructions::ToDouble => {
        to_double_func(&mut stack);
      }
      Instructions::Dup => {
        dup_func(&mut stack, ip).map_err(|e| e.format())?;
      }
      Instructions::Length => {
        length_func(&mut stack);
      }
      Instructions::SetProp(prop) => {
        set_prop_func(&mut stack, prop)?;
      }
      Instructions::Instantiate(class_name, argc) => {
        let instance = instantiate_func(&mut stack, &mut vars, class_name, *argc)?;
        stack.push(instance);
      }
      Instructions::Import(module_name, alias_idx) => {
        import_func(&mut vars, &options, module_name, *alias_idx)?;
      }
      Instructions::Break(target_ip) => {
        break_func(&mut ip, *target_ip);
        continue;
      }
      Instructions::Shrink => {
        let _ = shrink_func(&mut stack);
      }
      Instructions::Truncate => {
        let _ = truncate_func(&mut stack, ip).map_err(|e| e.format())?;
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
