/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::map_primitive::map_primitive;
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use half::f16;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use smol_str::SmolStr;
use std::sync::Arc;
use ts_rs::TS;
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[repr(u16)]
#[ts(export)]
pub enum Instructions {
  InitStack(u32),
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
  Neg(PrimitiveTypes),
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
impl Instructions {
  #[inline]
  pub fn from_parts(op: String, args: Vec<serde_json::Value>) -> Self {
    let mut op_lower = op;
    op_lower.make_ascii_lowercase();
    if args.is_empty() {
      return serde_json::from_value(serde_json::Value::String(op_lower))
        .unwrap_or(Instructions::Stop);
    }
    let json_payload = if args.len() == 1 {
      serde_json::json!({ &op_lower: args[0] })
    } else {
      serde_json::json!({ &op_lower: args })
    };
    serde_json::from_value(json_payload).unwrap_or_else(|_| {
      if op_lower == "push" {
        let val = &args[0];
        let value_internal = if let Some(i) = val.as_i64() {
          Value::Int64(i)
        } else if let Some(f) = val.as_f64() {
          Value::Float64(f)
        } else if let Some(b) = val.as_bool() {
          Value::Bool(b)
        } else if let Some(s) = val.as_str() {
          Value::String(SmolStr::new(s))
        } else {
          Value::Null
        };
        Instructions::Push(value_internal)
      } else {
        Instructions::Stop
      }
    })
  }
  #[inline]
  pub fn to_parts(&self) -> Vec<String> {
    let json = serde_json::to_value(self).unwrap_or(serde_json::Value::Null);
    if let Some(s) = json.as_str() {
      return vec![s.to_string()];
    }
    if let Some(obj) = json.as_object()
      && let Some((key, val)) = obj.iter().next()
    {
      let mut parts = vec![key.clone()];
      match val {
        JsonValue::Array(arr) => {
          for v in arr {
            if let Some(inner_arr) = v.as_array() {
              for inner_v in inner_arr {
                if let Some(s) = inner_v.as_str() {
                  parts.push(s.to_string());
                } else {
                  parts.push(inner_v.to_string());
                }
              }
            } else if let Some(s) = v.as_str() {
              parts.push(s.to_string());
            } else {
              parts.push(v.to_string());
            }
          }
        }
        _ => {
          if let Some(s) = val.as_str() {
            parts.push(s.to_string());
          } else {
            parts.push(val.to_string());
          }
        }
      }
      return parts;
    }
    vec!["Unknown".into()]
  }
  #[inline]
  pub fn from_json_array(item: &JsonValue) -> Result<Instructions, VMError> {
    if item.is_null() {
      return Ok(Instructions::Stop);
    }
    if let Some(s) = item.as_str() {
      return match s {
        "init_stack" => Ok(Instructions::InitStack(16)),
        "stop" => Ok(Instructions::Stop),
        "return" => Ok(Instructions::Return),
        "and" => Ok(Instructions::And),
        "or" => Ok(Instructions::Or),
        "xor" => Ok(Instructions::Xor),
        "not" => Ok(Instructions::Not),
        "print" => Ok(Instructions::Print),
        "println" => Ok(Instructions::Println),
        "stdout" => Ok(Instructions::Stdout),
        "stdoutln" => Ok(Instructions::Stdoutln),
        "stdin" => Ok(Instructions::Stdin),
        "clear_screen" => Ok(Instructions::ClearScreen),
        "break" => Ok(Instructions::Break(0)),
        "access_index" => Ok(Instructions::AccessIndex),
        "to_string" => Ok(Instructions::ToString),
        "to_short" => Ok(Instructions::ToShort),
        "to_integer" => Ok(Instructions::ToInteger),
        "to_long" => Ok(Instructions::ToLong),
        "to_octa" => Ok(Instructions::ToOcta),
        "to_half" => Ok(Instructions::ToHalf),
        "to_float" => Ok(Instructions::ToFloat),
        "to_double" => Ok(Instructions::ToDouble),
        "typeof" => Ok(Instructions::TypeOf),
        "inspect_obj" => Ok(Instructions::InspectObj),
        "inspect_arr" => Ok(Instructions::InspectArr),
        "length" => Ok(Instructions::Length),
        "concat" => Ok(Instructions::Concat),
        "dup" => Ok(Instructions::Dup),
        "swap" => Ok(Instructions::Swap),
        "truncate" => Ok(Instructions::Truncate),
        "shrink" => Ok(Instructions::Shrink),
        "make_obj" => Ok(Instructions::MakeObj(0)),
        "make_array" => Ok(Instructions::MakeArray(0)),
        _ => Err(VMError::InvalidOpcode {
          ip: 0,
          code: "UNKNOWN_OPCODE".into(),
        }),
      };
    }
    if item.is_object() {
      return Ok(Instructions::deserialize(item).unwrap_or(Instructions::Stop));
    }
    let arr = match item.as_array() {
      Some(a) => a,
      None => return Ok(Instructions::Stop),
    };
    let op = arr[0].as_str().ok_or_else(|| VMError::InvalidOpcode {
      ip: 0,
      code: "OPCODE_NOT_STRING".into(),
    })?;
    let op_bytes = op.as_bytes();
    let arg1 = arr.get(1);
    let arg2 = arr.get(2);
    match op_bytes {
      b"init_stack" => {
        let size = arg1.and_then(|v| v.as_u64()).unwrap_or(16) as u32;
        Ok(Instructions::InitStack(size))
      }
      b"push" => {
        let val = match arg1 {
          Some(v) => v,
          None => return Ok(Instructions::Stop),
        };
        let value_internal = if let Some(n) = val.as_i64() {
          if n >= i16::MIN as i64 && n <= i16::MAX as i64 {
            Value::Int16(n as i16)
          } else if n >= i32::MIN as i64 && n <= i32::MAX as i64 {
            Value::Int32(n as i32)
          } else {
            Value::Int64(n)
          }
        } else if let Some(f) = val.as_f64() {
          if f.is_nan() {
            Value::NaN
          } else if f == (f as f32) as f64 {
            if f == half::f16::from_f64(f).to_f64() {
              Value::Float16(half::f16::from_f64(f))
            } else {
              Value::Float32(f as f32)
            }
          } else {
            Value::Float64(f)
          }
        } else if let Some(b) = val.as_bool() {
          Value::Bool(b)
        } else if let Some(s) = val.as_str() {
          if let Ok(big_n) = s.parse::<i128>() {
            if big_n >= i64::MIN as i128 && big_n <= i64::MAX as i128 {
              Value::Int64(big_n as i64)
            } else {
              Value::Int128(big_n)
            }
          } else {
            Value::String(SmolStr::new(s))
          }
        } else if val.is_null() {
          Value::Null
        } else {
          Value::Undefined
        };
        Ok(Instructions::Push(value_internal))
      }
      b"add" => Ok(Instructions::Add(map_primitive(arg1))),
      b"sub" => Ok(Instructions::Sub(map_primitive(arg1))),
      b"mul" => Ok(Instructions::Mul(map_primitive(arg1))),
      b"div" => Ok(Instructions::Div(map_primitive(arg1))),
      b"mod" => Ok(Instructions::Mod(map_primitive(arg1))),
      b"shl" => Ok(Instructions::Shl(map_primitive(arg1))),
      b"shr" => Ok(Instructions::Shr(map_primitive(arg1))),
      b"ror" => Ok(Instructions::Ror(map_primitive(arg1))),
      b"rol" => Ok(Instructions::Rol(map_primitive(arg1))),
      b"sin" => Ok(Instructions::Sin(map_primitive(arg1))),
      b"cos" => Ok(Instructions::Cos(map_primitive(arg1))),
      b"tan" => Ok(Instructions::Tan(map_primitive(arg1))),
      b"neg" => Ok(Instructions::Neg(map_primitive(arg1))),
      b"pow" => Ok(Instructions::Pow(map_primitive(arg1))),
      b"powi" => Ok(Instructions::Powi(map_primitive(arg1))),
      b"powf" => Ok(Instructions::Powf(map_primitive(arg1))),
      b"gt" => Ok(Instructions::Gt(map_primitive(arg1))),
      b"lt" => Ok(Instructions::Lt(map_primitive(arg1))),
      b"ge" => Ok(Instructions::Ge(map_primitive(arg1))),
      b"le" => Ok(Instructions::Le(map_primitive(arg1))),
      b"eq" => Ok(Instructions::Eq(map_primitive(arg1))),
      b"neq" => Ok(Instructions::Neq(map_primitive(arg1))),
      b"and" => Ok(Instructions::And),
      b"or" => Ok(Instructions::Or),
      b"xor" => Ok(Instructions::Xor),
      b"not" => Ok(Instructions::Not),
      b"set" => {
        let s = arg1
          .and_then(|v| v.as_str())
          .ok_or(VMError::InvalidOpcode {
            ip: 0,
            code: "SET_MISSING_ARG".into(),
          })?;
        Ok(Instructions::Set(SmolStr::new(s)))
      }
      b"get" => {
        let s = arg1
          .and_then(|v| v.as_str())
          .ok_or(VMError::InvalidOpcode {
            ip: 0,
            code: "GET_MISSING_ARG".into(),
          })?;
        Ok(Instructions::Get(SmolStr::new(s)))
      }
      b"val" => {
        let s = arg1
          .and_then(|v| v.as_str())
          .ok_or(VMError::InvalidOpcode {
            ip: 0,
            code: "VAL_MISSING_ARG".into(),
          })?;
        Ok(Instructions::Val(SmolStr::new(s)))
      }
      b"access" => {
        let prop = arg1
          .and_then(|v| v.as_str())
          .expect("Access property must be a string");
        Ok(Instructions::Access(SmolStr::new(prop)))
      }
      b"print" => Ok(Instructions::Print),
      b"println" => Ok(Instructions::Println),
      b"stdout" => Ok(Instructions::Stdout),
      b"stdoutln" => Ok(Instructions::Stdoutln),
      b"stdin" => Ok(Instructions::Stdin),
      b"clear_screen" => Ok(Instructions::ClearScreen),
      b"if_false" => {
        let target = arg1
          .and_then(|v| v.as_u64())
          .expect("IF_FALSE jump target must be a number") as usize;
        Ok(Instructions::IfFalse(target))
      }
      b"jump" => {
        let target = arg1
          .and_then(|v| v.as_u64())
          .expect("Target jump must be a number") as usize;
        Ok(Instructions::Jump(target))
      }
      b"inc" => {
        let s = arg1.and_then(|v| v.as_str()).expect("Expected string");
        let num_type = match arg2.and_then(|v| v.as_str()) {
          Some("int") => PrimitiveTypes::Int,
          Some("flt") => PrimitiveTypes::Flt,
          Some("lng") => PrimitiveTypes::Lng,
          Some("dbl") => PrimitiveTypes::Dbl,
          Some("oct") => PrimitiveTypes::Oct,
          _ => PrimitiveTypes::Dbl,
        };
        Ok(Instructions::Inc(SmolStr::new(s), num_type))
      }
      b"dec" => {
        let s = arg1.and_then(|v| v.as_str()).expect("Expected string");
        let num_type = match arg2.and_then(|v| v.as_str()) {
          Some("int") => PrimitiveTypes::Int,
          Some("flt") => PrimitiveTypes::Flt,
          Some("lng") => PrimitiveTypes::Lng,
          Some("dbl") => PrimitiveTypes::Dbl,
          _ => PrimitiveTypes::Int,
        };
        Ok(Instructions::Dec(SmolStr::new(s), num_type))
      }
      b"func" => {
        let name = SmolStr::new(arg1.and_then(|v| v.as_str()).unwrap_or(""));
        let params = arg2.and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let start = arr.get(3).and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let end = arr.get(4).and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let mut names: Vec<SmolStr> = Vec::new();
        if let Some(v5) = arr.get(5) {
          if let Some(names_arr) = v5.as_array() {
            for n in names_arr {
              if let Some(s) = n.as_str() {
                names.push(SmolStr::new(s));
              }
            }
          } else if let Some(s) = v5.as_str() {
            names.push(SmolStr::new(s));
          }
        }
        for item in arr.iter().skip(6) {
          if let Some(s) = item.as_str() {
            names.push(SmolStr::new(s));
          }
        }
        Ok(Instructions::Func(name, params, start, end, names))
      }
      b"make_obj" => {
        let count = arg1
          .and_then(|v| v.as_u64())
          .expect("MakeObj count must be a number") as u32;
        Ok(Instructions::MakeObj(count))
      }
      b"make_array" => {
        let count = arg1
          .and_then(|v| v.as_u64())
          .expect("MakeArray count must be a number") as u32;
        Ok(Instructions::MakeArray(count))
      }
      b"to_string" => Ok(Instructions::ToString),
      b"to_short" => Ok(Instructions::ToShort),
      b"to_integer" => Ok(Instructions::ToInteger),
      b"to_long" => Ok(Instructions::ToLong),
      b"to_octa" => Ok(Instructions::ToOcta),
      b"to_half" => Ok(Instructions::ToHalf),
      b"to_float" => Ok(Instructions::ToFloat),
      b"to_double" => Ok(Instructions::ToDouble),
      b"dup" => Ok(Instructions::Dup),
      b"swap" => Ok(Instructions::Swap),
      b"truncate" => Ok(Instructions::Truncate),
      b"shrink" => Ok(Instructions::Shrink),
      b"length" => Ok(Instructions::Length),
      b"typeof" => Ok(Instructions::TypeOf),
      b"inspect_obj" => Ok(Instructions::InspectObj),
      b"inspect_arr" => Ok(Instructions::InspectArr),
      b"instantiate" => {
        let class_name = arg1
          .and_then(|v| v.as_str())
          .expect("ClassName must be string");
        let argc = arg2.and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        Ok(Instructions::Instantiate(SmolStr::new(class_name), argc))
      }
      b"set_prop" => {
        let prop = arg1
          .and_then(|v| v.as_str())
          .expect("set_prop property must be a string");
        Ok(Instructions::SetProp(SmolStr::new(prop)))
      }
      b"import" => {
        let module_name = arg1
          .and_then(|v| v.as_str())
          .expect("Module name must be string");
        let idx = arg2
          .and_then(|v| v.as_u64())
          .expect("Import alias index must be a number") as usize;
        Ok(Instructions::Import(SmolStr::new(module_name), idx))
      }
      b"break" => {
        let target = arg1.and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        Ok(Instructions::Jump(target))
      }
      b"access_index" => Ok(Instructions::AccessIndex),
      b"export" => Ok(Instructions::Export(
        arg1.and_then(|v| v.as_str()).unwrap_or("stop").into(),
      )),
      b"return" => Ok(Instructions::Return),
      b"call" => {
        let s = arg1.and_then(|v| v.as_str()).unwrap_or("stop");
        let argc = arg2
          .and_then(|v| v.as_u64())
          .expect("Argc must be a number") as u32;
        Ok(Instructions::Call(SmolStr::new(s), argc))
      }
      b"concat" => Ok(Instructions::Concat),
      b"stop" => Ok(Instructions::Stop),
      _ => Err(VMError::InvalidOpcode {
        ip: 0,
        code: "UNKNOWN_OPCODE".into(),
      }),
    }
  }
}
