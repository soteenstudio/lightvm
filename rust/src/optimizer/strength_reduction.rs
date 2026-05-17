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
#[inline(always)]
pub fn strength_reduction(bytecode: &mut Vec<Instructions>) {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(1) {
    let pair = (bytecode[i].clone(), bytecode[i + 1].clone());
    match pair {
      (Instructions::Push(v), Instructions::Mul(_)) if is_zero(&v) => {
        bytecode[i] = Instructions::Push(v);
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(v), Instructions::Mul(_)) if is_one(&v) => {
        bytecode[i] = Instructions::Nop;
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Mul(t))
        if n > 0 && (n & (n - 1)) == 0 =>
      {
        bytecode[i] = Instructions::Push(Value::Int32(n.trailing_zeros() as i32));
        bytecode[i + 1] = Instructions::Shl(t);
        i += 2;
      }
      (Instructions::Push(Value::Int64(n)), Instructions::Mul(t))
        if n > 0 && (n & (n - 1)) == 0 =>
      {
        bytecode[i] = Instructions::Push(Value::Int64(n.trailing_zeros() as i64));
        bytecode[i + 1] = Instructions::Shl(t);
        i += 2;
      }
      (Instructions::Push(v), Instructions::Div(_)) if is_one(&v) => {
        bytecode[i] = Instructions::Nop;
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(v), Instructions::Mod(_)) if is_one(&v) => {
        bytecode[i] = Instructions::Push(make_zero_like(&v));
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Mod(_))
        if n > 0 && (n & (n - 1)) == 0 =>
      {
        bytecode[i] = Instructions::Push(Value::Int32(n - 1));
        bytecode[i + 1] = Instructions::And;
        i += 2;
      }
      (Instructions::Push(Value::Int64(n)), Instructions::Mod(_))
        if n > 0 && (n & (n - 1)) == 0 =>
      {
        bytecode[i] = Instructions::Push(Value::Int64(n - 1));
        bytecode[i + 1] = Instructions::And;
        i += 2;
      }
      (Instructions::Push(v), Instructions::Add(_))
      | (Instructions::Push(v), Instructions::Sub(_))
        if is_zero(&v) =>
      {
        bytecode[i] = Instructions::Nop;
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      _ => i += 1,
    }
  }
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
}
fn is_zero(v: &Value) -> bool {
  match v {
    Value::Int32(n) => *n == 0,
    Value::Int64(n) => *n == 0,
    Value::Float32(n) => *n == 0.0,
    Value::Float64(n) => *n == 0.0,
    _ => false,
  }
}
fn is_one(v: &Value) -> bool {
  match v {
    Value::Int32(n) => *n == 1,
    Value::Int64(n) => *n == 1,
    Value::Float32(n) => *n == 1.0,
    Value::Float64(n) => *n == 1.0,
    _ => false,
  }
}
fn make_zero_like(v: &Value) -> Value {
  match v {
    Value::Int32(_) => Value::Int32(0),
    Value::Int64(_) => Value::Int64(0),
    Value::Float32(_) => Value::Float32(0.0),
    Value::Float64(_) => Value::Float64(0.0),
    _ => Value::Null,
  }
}
