/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instructions {
  Push(Value),
  Val(String),
  Set(String),
  Get(String),
  Add(PrimitiveTypes),
  Sub(PrimitiveTypes),
  Mul(PrimitiveTypes),
  Div(PrimitiveTypes),
  Mod(PrimitiveTypes),
  Gt(PrimitiveTypes),
  Lt(PrimitiveTypes),
  Ge(PrimitiveTypes),
  Le(PrimitiveTypes),
  Eq(PrimitiveTypes),
  Neq(PrimitiveTypes),
  And,
  Or,
  Print,
  Println,
  IfFalse(usize),
  Jump(usize),
  Inc(String),
  Dec(String),
  Call(String, u32),
  Func(String, u32, usize, usize, Vec<String>),
  Stop,
  Return,
  Break,
  Access(String),
  AccessIndex,
  ToString,
  ToNumber,
  ToInteger,
  ToFloat,
  MakeObj(u32),
  MakeArray(u32),
  TypeOf,
  Length,
  Concat,
  Dup,
  SetProp(String),
  Import(String, String),
  Export(String),
  Instantiate(String, u32),
}
impl Instructions {
  pub fn from_parts(op: String, args: Vec<serde_json::Value>) -> Self {
    match op.as_str() {
      "Push" => Instructions::Push(serde_json::from_value(args[0].clone()).unwrap()),
      "Set" => Instructions::Set(args[0].as_str().unwrap_or("").to_string()),
      "Get" => Instructions::Get(args[0].as_str().unwrap_or("").to_string()),
      "Jump" => Instructions::Jump(args[0].as_u64().unwrap_or(0) as usize),
      "Stop" => Instructions::Stop,
      _ => Instructions::Stop,
    }
  }
  pub fn to_parts(&self) -> Vec<String> {
    match self {
      Instructions::Push(v) => vec!["Push".to_string(), format!("{:?}", v)],
      Instructions::Set(s) => vec!["Set".to_string(), s.clone()],
      Instructions::Stop => vec!["Stop".to_string()],
      _ => vec!["Unknown".to_string()],
    }
  }
}
