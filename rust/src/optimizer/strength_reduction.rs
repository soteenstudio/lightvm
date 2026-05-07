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
pub fn strength_reduction(bytecode: &mut Vec<Instructions>) {
  let mut i = 0;
  while i < bytecode.len().saturating_sub(1) {
    let pair = (bytecode[i].clone(), bytecode[i + 1].clone());
    match pair {
      (Instructions::Push(Value::Int32(n)), Instructions::Mul(_)) if n == 0 => {
        bytecode[i] = Instructions::Push(Value::Int32(0));
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Mul(_)) if n == 1 => {
        bytecode[i] = Instructions::Nop;
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Mul(t))
        if n > 0 && (n & (n - 1)) == 0 =>
      {
        let shift_amount = n.trailing_zeros() as i32;
        bytecode[i] = Instructions::Push(Value::Int32(shift_amount));
        bytecode[i + 1] = Instructions::Shl(t);
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Div(_)) if n == 1 => {
        bytecode[i] = Instructions::Nop;
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Mod(_)) if n == 1 => {
        bytecode[i] = Instructions::Push(Value::Int32(0));
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      (Instructions::Push(Value::Int32(n)), Instructions::Mod(_))
        if n > 0 && (n & (n - 1)) == 0 =>
      {
        let mask = n - 1;
        bytecode[i] = Instructions::Push(Value::Int32(mask));
        bytecode[i + 1] = Instructions::And;
        i += 2;
      }
      (Instructions::Push(Value::Int32(0)), Instructions::Add(_))
      | (Instructions::Push(Value::Int32(0)), Instructions::Sub(_)) => {
        bytecode[i] = Instructions::Nop;
        bytecode[i + 1] = Instructions::Nop;
        i += 2;
      }
      _ => i += 1,
    }
  }
  bytecode.retain(|instr| !matches!(instr, Instructions::Nop));
}
