/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::add::{
  add_f16in::add_f16in, add_f32in::add_f32in, add_f64in::add_f64in, add_i16in::add_i16in,
  add_i32in::add_i32in, add_i64in::add_i64in, add_i128in::add_i128in,
};
use crate::instructions::math::sub::{
  sub_f16in::sub_f16in, sub_f32in::sub_f32in, sub_f64in::sub_f64in, sub_i16in::sub_i16in,
  sub_i32in::sub_i32in, sub_i64in::sub_i64in, sub_i128in::sub_i128in,
};
use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::types::{primitive_types::PrimitiveTypes, var_stack::VarStack};
use crate::utils::vmerror::VMError;
use half::f16;
#[inline]
pub fn inc_func(
  vars: &mut VarStack,
  stack: &mut Stack,
  index: usize,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  while vars.len() <= index {
    vars.push(Value::Undefined);
  }
  let var_ref = unsafe { vars.get_unchecked_mut(index) };
  if !var_ref.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Number",
      found: "Non-Number/Undefined",
    });
  }
  let result = match num_type {
    PrimitiveTypes::Sht => Value::Int16(add_i16in(var_ref.as_i16(), 1)),
    PrimitiveTypes::Int => Value::Int32(add_i32in(var_ref.as_i32(), 1)),
    PrimitiveTypes::Lng => Value::Int64(add_i64in(var_ref.as_i64(), 1)),
    PrimitiveTypes::Oct => Value::Int128(add_i128in(var_ref.as_i128(), 1)),
    PrimitiveTypes::Hlf => Value::Float16(add_f16in(var_ref.as_f16(), f16::ONE)),
    PrimitiveTypes::Flt => Value::Float32(add_f32in(var_ref.as_f32(), 1.0)),
    PrimitiveTypes::Dbl => Value::Float64(add_f64in(var_ref.as_f64(), 1.0)),
    _ => Value::Int32(add_i32in(var_ref.as_i32(), 1)),
  };
  *var_ref = result.clone();
  stack.push(result);
  Ok(())
}
#[inline]
pub fn dec_func(
  vars: &mut VarStack,
  index: usize,
  num_type: PrimitiveTypes,
  ip: usize,
) -> Result<(), VMError> {
  let var_ref = unsafe { vars.get_unchecked_mut(index) };
  if !var_ref.is_number() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Number",
      found: "Non-Number/Undefined",
    });
  }
  *var_ref = match num_type {
    PrimitiveTypes::Sht => Value::Int16(sub_i16in(var_ref.as_i16(), 1)),
    PrimitiveTypes::Int => Value::Int32(sub_i32in(var_ref.as_i32(), 1)),
    PrimitiveTypes::Lng => Value::Int64(sub_i64in(var_ref.as_i64(), 1)),
    PrimitiveTypes::Oct => Value::Int128(sub_i128in(var_ref.as_i128(), 1)),
    PrimitiveTypes::Hlf => Value::Float16(sub_f16in(var_ref.as_f16(), f16::ONE)),
    PrimitiveTypes::Flt => Value::Float32(sub_f32in(var_ref.as_f32(), 1.0)),
    PrimitiveTypes::Dbl => Value::Float64(sub_f64in(var_ref.as_f64(), 1.0)),
    _ => Value::Int32(sub_i32in(var_ref.as_i32(), 1)),
  };
  Ok(())
}
