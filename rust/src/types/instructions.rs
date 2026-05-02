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
use std::borrow::Cow;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Instructions {
  Push(Value),
  Val(Cow<'static, str>),
  ValIdx(usize),
  Set(Cow<'static, str>),
  SetIdx(usize),
  Get(Cow<'static, str>),
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
  Inc(Cow<'static, str>, PrimitiveTypes),
  IncIdx(usize, PrimitiveTypes),
  Dec(Cow<'static, str>, PrimitiveTypes),
  DecIdx(usize, PrimitiveTypes),
  Call(Cow<'static, str>, u32),
  Func(Cow<'static, str>, u32, usize, usize, Vec<Cow<'static, str>>),
  Stop,
  Return,
  Break,
  Access(Cow<'static, str>),
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
  SetProp(Cow<'static, str>),
  Import(Cow<'static, str>, Cow<'static, str>),
  Export(Cow<'static, str>),
  Instantiate(Cow<'static, str>, u32),
  Nop,
}
impl Instructions {
  pub fn from_parts(op: String, args: Vec<serde_json::Value>) -> Self {
    let op_lower = op.to_lowercase();
    if args.is_empty() {
      return serde_json::from_value(serde_json::Value::String(op_lower))
        .unwrap_or(Instructions::Stop);
    }
    let json_map = serde_json::json!({
        &op_lower: args[0]
    });
    serde_json::from_value(json_map).unwrap_or_else(|_| {
      if op_lower == "push" {
        let val = &args[0];
        let value_internal = if val.is_i64() {
          Value::Int64(val.as_i64().unwrap())
        } else if val.is_f64() {
          Value::Float64(val.as_f64().unwrap())
        } else if val.is_boolean() {
          Value::Bool(val.as_bool().unwrap())
        } else if val.is_string() {
          let s = val.as_str().unwrap();
          let leaked = Box::leak(s.to_owned().into_boxed_str());
          Value::String(Cow::Borrowed(leaked))
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
    if json.is_string() {
      return vec![json.as_str().unwrap().into()];
    } else if let Some(obj) = json.as_object() {
      let (key, val) = obj.iter().next().unwrap();
      let val_str = if key == "push" {
        if let Some(s) = val.as_str() {
          let clean_s = s.replace("\\\"", "\"").replace("\\\\", "\\");
          format!("\"{}\"", clean_s)
        } else if val.is_object() {
          let inner_obj = val.as_object().unwrap();
          let (_type_key, actual_val) = inner_obj.iter().next().unwrap();
          actual_val.as_str().unwrap_or("").to_string()
        } else {
          val.as_str().unwrap_or("").to_string()
        }
      } else {
        if let Some(s) = val.as_str() {
          s.into()
        } else {
          val.as_str().unwrap_or("").to_string()
        }
      };
      return vec![key.clone(), val_str];
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
        "length" => Instructions::Length,
        "concat" => Instructions::Concat,
        "dup" => Instructions::Dup,
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
          let s = val.as_str().unwrap();
          let leaked = Box::leak(s.to_owned().into_boxed_str());
          Value::String(Cow::Borrowed(leaked))
        } else if val.is_boolean() {
          Value::Bool(val.as_bool().unwrap())
        } else if val.is_string() {
          let s = val.as_str().expect("Push value must be a string");
          let leaked = Box::leak(s.to_owned().into_boxed_str());
          Value::String(Cow::Borrowed(leaked))
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
      "set" => {
        let s = arr[1].as_str().expect("Need string");
        let leaked: &'static str = Box::leak(s.to_owned().into_boxed_str());
        Instructions::Set(Cow::Borrowed(leaked))
      }
      "get" => {
        let s = arr[1].as_str().expect("Need string");
        let leaked: &'static str = Box::leak(s.to_owned().into_boxed_str());
        Instructions::Get(Cow::Borrowed(leaked))
      }
      "val" => {
        let s = arr[1].as_str().expect("Need string");
        let leaked: &'static str = Box::leak(s.to_owned().into_boxed_str());
        Instructions::Val(Cow::Borrowed(leaked))
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
        let leaked = Box::leak(s.to_owned().into_boxed_str());
        let num_type_str = arr.get(2).and_then(|v| v.as_str()).unwrap_or("int");
        let num_type = match num_type_str {
          "int" => PrimitiveTypes::Int,
          "flt" => PrimitiveTypes::Flt,
          "lng" => PrimitiveTypes::Lng,
          "dbl" => PrimitiveTypes::Dbl,
          _ => PrimitiveTypes::Dbl,
        };
        Instructions::Inc(Cow::Borrowed(leaked), num_type)
      }
      "dec" => {
        let s = arr[1].as_str().expect("Expected string");
        let leaked = Box::leak(s.to_owned().into_boxed_str());
        let num_type_str = arr.get(2).and_then(|v| v.as_str()).unwrap_or("int");
        let num_type = match num_type_str {
          "int" => PrimitiveTypes::Int,
          "flt" => PrimitiveTypes::Flt,
          "lng" => PrimitiveTypes::Lng,
          "dbl" => PrimitiveTypes::Dbl,
          _ => PrimitiveTypes::Int,
        };
        Instructions::Inc(Cow::Borrowed(leaked), num_type)
      }
      "func" => {
        let name_str = arr[1].as_str().unwrap();
        let name = Cow::Borrowed(Box::leak(name_str.to_owned().into_boxed_str()));
        let params = arr[2].as_u64().unwrap() as u32;
        let start = arr[3].as_u64().unwrap() as usize;
        let end = arr[4].as_u64().unwrap() as usize;
        let mut names: Vec<Cow<'static, str>> = arr[5]
          .as_array()
          .unwrap()
          .iter()
          .map(|n| {
            let s = n.as_str().unwrap();
            Cow::Borrowed(Box::leak(s.to_owned().into_boxed_str()))
          })
          .collect();
        for i in 6..arr.len() {
          names.push(Cow::Owned(
            arr[i].as_str().expect("Expected string").to_owned(),
          ));
        }
        Instructions::Func(name, params, start, end, names)
      }
      "export" => Instructions::Export(arr[1].as_str().unwrap().to_owned().into()),
      "return" => Instructions::Return,
      "call" => {
        let s = arr[1].as_str().unwrap();
        let leaked = Box::leak(s.to_owned().into_boxed_str());
        let argc = arr[2].as_u64().expect("Argc must be a number") as u32;
        Instructions::Call(Cow::Borrowed(leaked), argc)
      }
      "concat" => Instructions::Concat,
      "stop" => Instructions::Stop,
      _ => panic!("Opcode '{}' belum di-map atau sampah!", op),
    }
  }
}
