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
use ahash::AHashMap;
use smol_str::SmolStr;
#[inline]
#[cold]
pub fn resolve_symbols(bytecode: &mut [Instructions]) -> usize {
  let mut symbol_table: AHashMap<SmolStr, usize> = AHashMap::new();
  let mut next_idx = 0;
  for instr in bytecode.iter_mut() {
    match instr {
      Instructions::Push(val) => match val {
        Value::Int16(v) => *instr = Instructions::PushInt16(*v),
        Value::Int32(v) => *instr = Instructions::PushInt32(*v),
        Value::Int64(v) => *instr = Instructions::PushInt64(*v),
        Value::Int128(v) => *instr = Instructions::PushInt128(*v),
        Value::Float16(v) => *instr = Instructions::PushFloat16(*v),
        Value::Float32(v) => *instr = Instructions::PushFloat32(*v),
        Value::Float64(v) => *instr = Instructions::PushFloat64(*v),
        Value::Bool(v) => *instr = Instructions::PushBool(*v),
        Value::Undefined => *instr = Instructions::PushUndefined,
        _ => {}
      },
      Instructions::Get(name) => {
        let idx = *symbol_table.entry(name.clone()).or_insert_with(|| {
          let i = next_idx;
          next_idx += 1;
          i
        });
        *instr = Instructions::GetIdx(idx);
      }
      Instructions::Set(name) => {
        let idx = *symbol_table.entry(name.clone()).or_insert_with(|| {
          let i = next_idx;
          next_idx += 1;
          i
        });
        *instr = Instructions::SetIdx(idx);
      }
      Instructions::Inc(name, num_type) => {
        let idx = *symbol_table.entry(name.clone()).or_insert_with(|| {
          let i = next_idx;
          next_idx += 1;
          i
        });
        *instr = Instructions::IncIdx(idx, *num_type);
      }
      _ => {}
    }
    println!("{:?}", instr);
  }
  next_idx
}
