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
use crate::utils::map_primitive::map_primitive;
use half::f16;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use smol_str::SmolStr;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u16)]
pub enum Instructions {
  InitStack(u32),
  PushInt16(i16),
  PushInt32(i32),
  PushInt64(i64),
  PushInt128(i128),
  PushFloat16(f16),
  PushFloat32(f32),
  PushFloat64(f64),
  PushBool(bool),
  PushUndefined,
  Push(Value),
  Val(SmolStr),
  ValIdx(usize),
  Set(SmolStr),
  SetIdx(usize),
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
  IfFalse(usize),
  Jump(usize),
  Inc(SmolStr, PrimitiveTypes),
  IncIdx(usize, PrimitiveTypes),
  Dec(SmolStr, PrimitiveTypes),
  DecIdx(usize, PrimitiveTypes),
  Call(SmolStr, u32),
  Func(SmolStr, u32, usize, usize, Vec<SmolStr>),
  Stop,
  Return,
  Break(usize),
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
  SetProp(SmolStr),
  Import(SmolStr, usize),
  Export(SmolStr),
  Instantiate(SmolStr, u32),
  Nop,
  Truncate,
  Shrink,
}
impl Instructions {
  pub fn from_parts(op: String, args: Vec<serde_json::Value>) -> Self {
    let op_lower = op.to_lowercase();
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
        let value_internal = if val.is_i64() {
          Value::Int64(val.as_i64().unwrap())
        } else if val.is_f64() {
          Value::Float64(val.as_f64().unwrap())
        } else if val.is_boolean() {
          Value::Bool(val.as_bool().unwrap())
        } else if val.is_string() {
          Value::String(SmolStr::new(val.as_str().unwrap()))
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
    if let Some(obj) = json.as_object() {
      if let Some((key, val)) = obj.iter().next() {
        let mut parts = vec![key.clone()];
        match val {
          JsonValue::Array(arr) => {
            for v in arr {
              if let Some(inner_arr) = v.as_array() {
                for inner_v in inner_arr {
                  parts.push(inner_v.as_str().unwrap_or("").replace("\"", ""));
                }
              } else {
                parts.push(v.to_string().replace("\"", ""));
              }
            }
          }
          _ => parts.push(val.to_string().replace("\"", "")),
        }
        return parts;
      }
    }
    vec!["Unknown".into()]
  }
  #[inline]
  pub fn from_json_array(item: &JsonValue) -> Self {
    if item.is_null() {
      return Instructions::Stop;
    }
    if let Some(s) = item.as_str() {
      return match s {
        "init_stack" => Instructions::InitStack(16),
        "stop" => Instructions::Stop,
        "return" => Instructions::Return,
        "and" => Instructions::And,
        "or" => Instructions::Or,
        "xor" => Instructions::Xor,
        "not" => Instructions::Not,
        "print" => Instructions::Print,
        "println" => Instructions::Println,
        "break" => Instructions::Break(0),
        "accessindex" => Instructions::AccessIndex,
        "to_string" => Instructions::ToString,
        "to_short" => Instructions::ToShort,
        "to_integer" => Instructions::ToInteger,
        "to_long" => Instructions::ToLong,
        "to_octa" => Instructions::ToOcta,
        "to_half" => Instructions::ToHalf,
        "to_float" => Instructions::ToFloat,
        "to_double" => Instructions::ToDouble,
        "typeof" => Instructions::TypeOf,
        "inspect_obj" => Instructions::InspectObj,
        "inspect_arr" => Instructions::InspectArr,
        "length" => Instructions::Length,
        "concat" => Instructions::Concat,
        "dup" => Instructions::Dup,
        "truncate" => Instructions::Truncate,
        "shrink" => Instructions::Shrink,
        "make_obj" => Instructions::MakeObj(0),
        "make_array" => Instructions::MakeArray(0),
        _ => Instructions::Stop,
      };
    }
    if item.is_object() {
      return serde_json::from_value(item.clone()).unwrap_or(Instructions::Stop);
    }
    let arr = match item.as_array() {
      Some(a) => a,
      None => {
        return Instructions::Stop;
      }
    };
    let op = arr[0].as_str().expect("Opcode must be a string");
    let op_bytes = op.as_bytes();
    let arg1 = arr.get(1);
    let arg2 = arr.get(2);
    match op_bytes {
      b"init_stack" => {
        let size = arg1.and_then(|v| v.as_u64()).unwrap_or(16) as u32;
        Instructions::InitStack(size)
      }
      b"push" => {
        let val = match arg1 {
          Some(v) => v,
          None => return Instructions::Stop,
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
          if f == (f as f32) as f64 {
            if f == half::f16::from_f64(f).to_f64() {
              Value::Float16(half::f16::from_f64(f))
            } else {
              Value::Float32(f as f32)
            }
          } else {
            Value::Float64(f)
          }
        } else if val.is_boolean() {
          Value::Bool(val.as_bool().unwrap_or(false))
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
        Instructions::Push(value_internal)
      }
      b"add" => Instructions::Add(map_primitive(arg1)),
      b"sub" => Instructions::Sub(map_primitive(arg1)),
      b"mul" => Instructions::Mul(map_primitive(arg1)),
      b"div" => Instructions::Div(map_primitive(arg1)),
      b"mod" => Instructions::Mod(map_primitive(arg1)),
      b"shl" => Instructions::Shl(map_primitive(arg1)),
      b"shr" => Instructions::Shr(map_primitive(arg1)),
      b"ror" => Instructions::Ror(map_primitive(arg1)),
      b"rol" => Instructions::Rol(map_primitive(arg1)),
      b"sin" => Instructions::Sin(map_primitive(arg1)),
      b"cos" => Instructions::Cos(map_primitive(arg1)),
      b"tan" => Instructions::Tan(map_primitive(arg1)),
      b"pow" => Instructions::Pow(map_primitive(arg1)),
      b"powi" => Instructions::Powi(map_primitive(arg1)),
      b"powf" => Instructions::Powf(map_primitive(arg1)),
      b"gt" => Instructions::Gt(map_primitive(arg1)),
      b"lt" => Instructions::Lt(map_primitive(arg1)),
      b"ge" => Instructions::Ge(map_primitive(arg1)),
      b"le" => Instructions::Le(map_primitive(arg1)),
      b"eq" => Instructions::Eq(map_primitive(arg1)),
      b"neq" => Instructions::Neq(map_primitive(arg1)),
      b"and" => Instructions::And,
      b"or" => Instructions::Or,
      b"xor" => Instructions::Xor,
      b"not" => Instructions::Not,
      b"set" => Instructions::Set(SmolStr::new(
        arg1.and_then(|v| v.as_str()).expect("Need string"),
      )),
      b"get" => Instructions::Get(SmolStr::new(
        arg1.and_then(|v| v.as_str()).expect("Need string"),
      )),
      b"val" => Instructions::Val(SmolStr::new(
        arg1.and_then(|v| v.as_str()).expect("Need string"),
      )),
      b"access" => {
        let prop = arg1
          .and_then(|v| v.as_str())
          .expect("Access property must be a string");
        Instructions::Access(SmolStr::new(prop))
      }
      b"print" => Instructions::Print,
      b"println" => Instructions::Println,
      b"if_false" => {
        let target = arg1
          .and_then(|v| v.as_u64())
          .expect("Target jump IF_FALSE harus angka") as usize;
        Instructions::IfFalse(target)
      }
      b"jump" => {
        let target = arg1
          .and_then(|v| v.as_u64())
          .expect("Target jump harus angka") as usize;
        Instructions::Jump(target)
      }
      b"inc" => {
        let s = arg1.and_then(|v| v.as_str()).expect("Expected string");
        let num_type_str = arg2.and_then(|v| v.as_str()).unwrap_or("int");
        let num_type = match num_type_str {
          "int" => PrimitiveTypes::Int,
          "flt" => PrimitiveTypes::Flt,
          "lng" => PrimitiveTypes::Lng,
          "dbl" => PrimitiveTypes::Dbl,
          "oct" => PrimitiveTypes::Oct,
          _ => PrimitiveTypes::Dbl,
        };
        Instructions::Inc(SmolStr::new(s), num_type)
      }
      b"dec" => {
        let s = arg1.and_then(|v| v.as_str()).expect("Expected string");
        let num_type_str = arg2.and_then(|v| v.as_str()).unwrap_or("int");
        let num_type = match num_type_str {
          "int" => PrimitiveTypes::Int,
          "flt" => PrimitiveTypes::Flt,
          "lng" => PrimitiveTypes::Lng,
          "dbl" => PrimitiveTypes::Dbl,
          _ => PrimitiveTypes::Int,
        };
        Instructions::Dec(SmolStr::new(s), num_type)
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
        Instructions::Func(name, params, start, end, names)
      }
      b"make_obj" => {
        let count = arg1
          .and_then(|v| v.as_u64())
          .expect("MakeObj count harus angka") as u32;
        Instructions::MakeObj(count)
      }
      b"make_array" => {
        let count = arg1
          .and_then(|v| v.as_u64())
          .expect("MakeArray count harus angka") as u32;
        Instructions::MakeArray(count)
      }
      b"to_string" => Instructions::ToString,
      b"to_short" => Instructions::ToShort,
      b"to_integer" => Instructions::ToInteger,
      b"to_long" => Instructions::ToLong,
      b"to_octa" => Instructions::ToOcta,
      b"to_half" => Instructions::ToHalf,
      b"to_float" => Instructions::ToFloat,
      b"to_double" => Instructions::ToDouble,
      b"dup" => Instructions::Dup,
      b"truncate" => Instructions::Truncate,
      b"shrink" => Instructions::Shrink,
      b"length" => Instructions::Length,
      b"typeof" => Instructions::TypeOf,
      b"inspect_obj" => Instructions::InspectObj,
      b"inspect_arr" => Instructions::InspectArr,
      b"instantiate" => {
        let class_name = arg1
          .and_then(|v| v.as_str())
          .expect("ClassName must be string");
        let argc = arg2.and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        Instructions::Instantiate(SmolStr::new(class_name), argc)
      }
      b"set_prop" => {
        let prop = arg1
          .and_then(|v| v.as_str())
          .expect("set_prop property must be a string");
        Instructions::SetProp(SmolStr::new(prop))
      }
      b"import" => {
        let module_name = arg1
          .and_then(|v| v.as_str())
          .expect("Module name must be string");
        let idx = arg2
          .and_then(|v| v.as_u64())
          .expect("Import alias index must be a number") as usize;
        Instructions::Import(SmolStr::new(module_name), idx)
      }
      b"break" => {
        let target = arg1.and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        Instructions::Jump(target)
      }
      b"access_index" => Instructions::AccessIndex,
      b"export" => Instructions::Export(
        arg1
          .and_then(|v| v.as_str())
          .unwrap_or("stop")
          .to_owned()
          .into(),
      ),
      b"return" => Instructions::Return,
      b"call" => {
        let s = arg1.and_then(|v| v.as_str()).unwrap_or("stop");
        let argc = arg2
          .and_then(|v| v.as_u64())
          .expect("Argc must be a number") as u32;
        Instructions::Call(SmolStr::new(s), argc)
      }
      b"concat" => Instructions::Concat,
      b"stop" => Instructions::Stop,
      _ => Instructions::Nop,
    }
  }
}
