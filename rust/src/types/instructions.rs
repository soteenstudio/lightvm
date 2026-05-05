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
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use smol_str::SmolStr;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Instructions {
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
  Inc(SmolStr, PrimitiveTypes),
  IncIdx(usize, PrimitiveTypes),
  Dec(SmolStr, PrimitiveTypes),
  DecIdx(usize, PrimitiveTypes),
  Call(SmolStr, u32),
  Func(SmolStr, u32, usize, usize, Vec<SmolStr>),
  Stop,
  Return,
  Break,
  Access(SmolStr),
  AccessIndex,
  ToString,
  ToNumber,
  ToInteger,
  ToFloat,
  MakeObj(u32),
  MakeArray(u32),
  TypeOf,
  InspectObj,
  InspectArr,
  Length,
  Concat,
  Dup,
  SetProp(SmolStr),
  Import(SmolStr, SmolStr),
  Export(SmolStr),
  Instantiate(SmolStr, u32),
  Nop,
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
  pub fn from_json_array(item: &JsonValue) -> Self {
    if item.is_null() {
      return Instructions::Stop;
    }
    if let Some(s) = item.as_str() {
      return match s {
        "stop" => Instructions::Stop,
        "return" => Instructions::Return,
        "and" => Instructions::And,
        "or" => Instructions::Or,
        "print" => Instructions::Print,
        "println" => Instructions::Println,
        "break" => Instructions::Break,
        "accessindex" => Instructions::AccessIndex,
        "tostring" => Instructions::ToString,
        "tonumber" => Instructions::ToNumber,
        "tointeger" => Instructions::ToInteger,
        "tofloat" => Instructions::ToFloat,
        "typeof" => Instructions::TypeOf,
        "inspect_obj" => Instructions::InspectObj,
        "inspect_arr" => Instructions::InspectArr,
        "length" => Instructions::Length,
        "concat" => Instructions::Concat,
        "dup" => Instructions::Dup,
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
    match op {
      "push" => {
        let val = &arr[1];
        let value_internal = if val.is_i64() {
          Value::Int64(val.as_i64().unwrap())
        } else if val.is_f64() {
          Value::Float64(val.as_f64().unwrap())
        } else if val.is_boolean() {
          Value::Bool(val.as_bool().unwrap())
        } else if val.is_string() {
          Value::String(SmolStr::new(val.as_str().unwrap()))
        } else if val.is_null() {
          Value::Null
        } else {
          Value::Undefined
        };
        Instructions::Push(value_internal)
      }
      "add" => Instructions::Add(map_primitive(arr.get(1))),
      "sub" => Instructions::Sub(map_primitive(arr.get(1))),
      "mul" => Instructions::Mul(map_primitive(arr.get(1))),
      "div" => Instructions::Div(map_primitive(arr.get(1))),
      "mod" => Instructions::Mod(map_primitive(arr.get(1))),
      "gt" => Instructions::Gt(map_primitive(arr.get(1))),
      "lt" => Instructions::Lt(map_primitive(arr.get(1))),
      "ge" => Instructions::Ge(map_primitive(arr.get(1))),
      "le" => Instructions::Le(map_primitive(arr.get(1))),
      "eq" => Instructions::Eq(map_primitive(arr.get(1))),
      "neq" => Instructions::Neq(map_primitive(arr.get(1))),
      "and" => Instructions::And,
      "or" => Instructions::Or,
      "set" => Instructions::Set(SmolStr::new(arr[1].as_str().expect("Need string"))),
      "get" => Instructions::Get(SmolStr::new(arr[1].as_str().expect("Need string"))),
      "val" => Instructions::Val(SmolStr::new(arr[1].as_str().expect("Need string"))),
      "access" => {
        let prop = arr[1].as_str().expect("Access property must be a string");
        Instructions::Access(SmolStr::new(prop))
      }
      "print" => Instructions::Print,
      "println" => Instructions::Println,
      "if_false" => {
        let target = arr[1].as_u64().expect("Target jump IF_FALSE harus angka") as usize;
        Instructions::IfFalse(target)
      }
      "jump" => {
        let target = arr[1].as_u64().expect("Target jump harus angka") as usize;
        Instructions::Jump(target)
      }
      "inc" => {
        let s = arr[1].as_str().expect("Expected string");
        let num_type_str = arr.get(2).and_then(|v| v.as_str()).unwrap_or("int");
        let num_type = match num_type_str {
          "int" => PrimitiveTypes::Int,
          "flt" => PrimitiveTypes::Flt,
          "lng" => PrimitiveTypes::Lng,
          "dbl" => PrimitiveTypes::Dbl,
          _ => PrimitiveTypes::Dbl,
        };
        Instructions::Inc(SmolStr::new(s), num_type)
      }
      "dec" => {
        let s = arr[1].as_str().expect("Expected string");
        let num_type_str = arr.get(2).and_then(|v| v.as_str()).unwrap_or("int");
        let num_type = match num_type_str {
          "int" => PrimitiveTypes::Int,
          "flt" => PrimitiveTypes::Flt,
          "lng" => PrimitiveTypes::Lng,
          "dbl" => PrimitiveTypes::Dbl,
          _ => PrimitiveTypes::Int,
        };
        Instructions::Dec(SmolStr::new(s), num_type)
      }
      "func" => {
        let name = SmolStr::new(arr.get(1).and_then(|v| v.as_str()).unwrap_or(""));
        let params = arr.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as u32;
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
        for i in 6..arr.len() {
          if let Some(s) = arr[i].as_str() {
            names.push(SmolStr::new(s));
          }
        }
        Instructions::Func(name, params, start, end, names)
      }
      "make_obj" => {
        let count = arr[1].as_u64().expect("MakeObj count harus angka") as u32;
        Instructions::MakeObj(count)
      }
      "make_array" => {
        let count = arr[1].as_u64().expect("MakeArray count harus angka") as u32;
        Instructions::MakeArray(count)
      }
      "typeof" => Instructions::TypeOf,
      "inspect_obj" => Instructions::InspectObj,
      "inspect_arr" => Instructions::InspectArr,
      "instantiate" => {
        let class_name = arr[1].as_str().expect("ClassName must be string");
        let argc = arr[2].as_u64().unwrap_or(0) as u32;
        Instructions::Instantiate(SmolStr::new(class_name), argc)
      }
      "access_index" => Instructions::AccessIndex,
      "export" => Instructions::Export(arr[1].as_str().unwrap().to_owned().into()),
      "return" => Instructions::Return,
      "call" => {
        let s = arr[1].as_str().unwrap();
        let argc = arr[2].as_u64().expect("Argc must be a number") as u32;
        Instructions::Call(SmolStr::new(s), argc)
      }
      "concat" => Instructions::Concat,
      "stop" => Instructions::Stop,
      _ => panic!("Opcode '{}' belum di-map atau sampah!", op),
    }
  }
}
