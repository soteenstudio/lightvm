/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use ahash::AHashMap;
use half::f16;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::sync::Arc;
use ts_rs::TS;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "snake_case")]
#[repr(u16)]
#[ts(export)]
pub enum Instructions {
  PushInt16(i16),
  PushInt32(i32),
  PushInt64(i64),
  #[ts(type = "number")]
  PushInt128(i128),
  #[ts(type = "number")]
  PushFloat16(f16),
  PushFloat32(f32),
  PushFloat64(f64),
  #[ts(type = "string")]
  PushString(SmolStr),
  #[ts(type = "any[]")]
  PushArray(Arc<Vec<Value>>),
  #[ts(type = "Record<string, any>")]
  PushObject(Arc<AHashMap<SmolStr, Value>>),
  PushBool(bool),
  PushNull,
  PushUndefined,
  PushNaN,
  Push(Value),
  #[ts(type = "string")]
  Val(SmolStr),
  ValIdx(usize),
  #[ts(type = "string")]
  Set(SmolStr),
  SetIdx(usize),
  #[ts(type = "string")]
  Get(SmolStr),
  GetIdx(usize),
  Add(PrimitiveTypes),
  Sub(PrimitiveTypes),
  Mul(PrimitiveTypes),
  Div(PrimitiveTypes),
  Mod(PrimitiveTypes),
  Shl(PrimitiveTypes),
  Shr(PrimitiveTypes),
  Ror(PrimitiveTypes),
  Rol(PrimitiveTypes),
  Sin(PrimitiveTypes),
  Cos(PrimitiveTypes),
  Tan(PrimitiveTypes),
  Asin(PrimitiveTypes),
  Acos(PrimitiveTypes),
  Atan(PrimitiveTypes),
  Atan2(PrimitiveTypes),
  Sinh(PrimitiveTypes),
  Cosh(PrimitiveTypes),
  Tanh(PrimitiveTypes),
  Asinh(PrimitiveTypes),
  Acosh(PrimitiveTypes),
  Atanh(PrimitiveTypes),
  Sqrt(PrimitiveTypes),
  Cbrt(PrimitiveTypes),
  Neg(PrimitiveTypes),
  Ln(PrimitiveTypes),
  Exp(PrimitiveTypes),
  Log2(PrimitiveTypes),
  Log10(PrimitiveTypes),
  Pow(PrimitiveTypes),
  Powi(PrimitiveTypes),
  Powf(PrimitiveTypes),
  Gt(PrimitiveTypes),
  Lt(PrimitiveTypes),
  Ge(PrimitiveTypes),
  Le(PrimitiveTypes),
  Eq(PrimitiveTypes),
  Neq(PrimitiveTypes),
  And,
  Or,
  Xor,
  Not,
  Print,
  Println,
  Stdout,
  Stdoutln,
  Stdin,
  ClearScreen,
  IfFalse(usize),
  Jump(usize),
  #[ts(type = "[string, PrimitiveTypes]")]
  Inc(SmolStr, PrimitiveTypes),
  IncIdx(usize, PrimitiveTypes),
  #[ts(type = "[string, PrimitiveTypes]")]
  Dec(SmolStr, PrimitiveTypes),
  DecIdx(usize, PrimitiveTypes),
  #[ts(type = "[string, PrimitiveTypes]")]
  Call(SmolStr, u32),
  #[ts(type = "[string, number, number, number, string[]]")]
  Func(SmolStr, u32, usize, usize, Vec<SmolStr>),
  Stop,
  Return,
  Break(usize),
  #[ts(type = "string")]
  Access(SmolStr),
  AccessIndex,
  ToString,
  ToShort,
  ToInteger,
  ToLong,
  ToOcta,
  ToHalf,
  ToFloat,
  ToDouble,
  MakeObj(u32),
  MakeArray(u32),
  TypeOf,
  InspectObj,
  InspectArr,
  Length,
  Concat,
  Dup,
  Swap,
  #[ts(type = "string")]
  SetProp(SmolStr),
  #[ts(type = "[string, number]")]
  Import(SmolStr, usize),
  #[ts(type = "string")]
  Export(SmolStr),
  #[ts(type = "[string, number]")]
  Instantiate(SmolStr, u32),
  Nop,
  Truncate,
  Shrink,
}
