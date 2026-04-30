/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::math::add::{
  add_f32in::add_f32in, add_f64in::add_f64in, add_i32in::add_i32in, add_i64in::add_i64in,
};
use crate::instructions::math::sub::sub_i32in::sub_i32in;
use crate::types::value::Value;
use std::collections::HashMap;
pub fn inc_func(
  vars: &mut HashMap<String, Value>,
  stack: &mut Vec<Value>,
  var_name: String,
  num_type: String,
  is_hot: bool,
) -> Result<(), String> {
  let old_val = vars.get(&var_name).cloned().unwrap_or(Value::Undefined);
  if !old_val.is_number() {
    return Err(format!(
      "TypeError: Cannot increment non-number variable '{}'",
      var_name
    ));
  }
  let result = if is_hot {
    match num_type.as_str() {
      "int" => Value::Int32(add_i32in(old_val.as_i32(), 1)),
      "lng" => Value::Int64(add_i64in(old_val.as_i64(), 1)),
      "flt" => Value::Float32(add_f32in(old_val.as_f32(), 1.0)),
      "dbl" => Value::Float64(add_f64in(old_val.as_f64(), 1.0)),
      _ => Value::Int32(add_i32in(old_val.as_i32(), 1)),
    }
  } else {
    Value::Int32(add_i32in(old_val.as_i32(), 1))
  };
  vars.insert(var_name, result.clone());
  stack.push(result);
  Ok(())
}
pub fn dec_func(vars: &mut HashMap<String, Value>, var_name: String) -> Result<(), String> {
  let old_val = vars.get(&var_name).cloned().unwrap_or(Value::Undefined);
  if !old_val.is_number() {
    return Err(format!(
      "TypeError: Cannot decrement non-number variable '{}'",
      var_name
    ));
  }
  let result = Value::Int32(sub_i32in(old_val.as_i32(), 1));
  vars.insert(var_name, result);
  Ok(())
}
