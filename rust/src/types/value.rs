/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::fast_format::{float_to_cow, int_to_cow};
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
  Int16(i16),
  Int32(i32),
  Int64(i64),
  Float32(f32),
  Float64(f64),
  String(SmolStr),
  Array(Vec<Value>),
  Object(AHashMap<SmolStr, Value>),
  Bool(bool),
  Null,
  Undefined,
  Marker(SmolStr),
}
#[derive(Clone)]
pub struct FuncMetadata {
  pub params_count: u32,
  pub param_names: Vec<SmolStr>,
  pub start: usize,
  pub end: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOptions {
  pub entry: Option<usize>,
  pub args: Vec<Value>,
  pub capture_return: bool,
  pub imports: AHashMap<SmolStr, Value>,
}
impl Value {
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Bool(v) => *v,
      Value::Int16(v) => *v != 0,
      Value::Int32(v) => *v != 0,
      Value::Int64(v) => *v != 0,
      Value::Float32(v) => *v != 0.0 && !v.is_nan(),
      Value::Float64(v) => *v != 0.0 && !v.is_nan(),
      Value::String(v) => !v.is_empty(),
      Value::Null | Value::Undefined => false,
      Value::Marker(_) => true,
      Value::Array(_) | Value::Object(_) => true,
    }
  }
  pub fn as_bool_refined(&self) -> bool {
    self.is_truthy()
  }
  pub fn is_number(&self) -> bool {
    match self {
      Value::Int32(_) | Value::Int64(_) | Value::Float32(_) | Value::Float64(_) => true,
      _ => false,
    }
  }
  pub fn as_i16(&self) -> i16 {
    match self {
      Value::Int16(v) => *v,
      Value::Int32(v) => *v as i16,
      Value::Int64(v) => *v as i16,
      Value::Float32(v) => *v as i16,
      Value::Float64(v) => *v as i16,
      _ => panic!("Expected Int16 compatible value, found {:?}", self),
    }
  }
  pub fn as_i32(&self) -> i32 {
    match self {
      Value::Int32(v) => *v,
      Value::Int16(v) => *v as i32,
      Value::Int64(v) => *v as i32,
      Value::Float32(v) => *v as i32,
      Value::Float64(v) => *v as i32,
      _ => panic!("Expected Int32 compatible value, found {:?}", self),
    }
  }
  pub fn as_i64(&self) -> i64 {
    match self {
      Value::Int64(v) => *v,
      Value::Int16(v) => *v as i64,
      Value::Int32(v) => *v as i64,
      Value::Float32(v) => *v as i64,
      Value::Float64(v) => *v as i64,
      _ => panic!("Expected Int64 compatible value, found {:?}", self),
    }
  }
  pub fn as_f32(&self) -> f32 {
    match self {
      Value::Float32(v) => *v,
      Value::Float64(v) => *v as f32,
      Value::Int16(v) => *v as f32,
      Value::Int32(v) => *v as f32,
      Value::Int64(v) => *v as f32,
      _ => panic!("Expected Float32 compatible value, found {:?}", self),
    }
  }
  pub fn as_f64(&self) -> f64 {
    match self {
      Value::Float64(v) => *v,
      Value::Float32(v) => *v as f64,
      Value::Int16(v) => *v as f64,
      Value::Int32(v) => *v as f64,
      Value::Int64(v) => *v as f64,
      _ => panic!("Expected Float64 compatible value, found {:?}", self),
    }
  }
  pub fn as_string(&self) -> SmolStr {
    match self {
      Value::String(v) => v.clone(),
      Value::Int16(v) => int_to_cow(*v as i64).into(),
      Value::Int32(v) => int_to_cow(*v as i64).into(),
      Value::Int64(v) => int_to_cow(*v).into(),
      Value::Float32(v) => float_to_cow(*v as f64).into(),
      Value::Float64(v) => float_to_cow(*v).into(),
      Value::Bool(v) => SmolStr::new(if *v { "true" } else { "false" }),
      Value::Null => SmolStr::new("null"),
      Value::Undefined => SmolStr::new("undefined"),
      Value::Marker(v) => v.clone(),
      Value::Array(_) => SmolStr::new_static("[array]"),
      Value::Object(_) => SmolStr::new_static("[object]"),
    }
  }
  pub fn as_bool(&self) -> bool {
    self.is_truthy()
  }
  pub fn type_of(&self) -> &'static str {
    match self {
      Value::Int16(_) => "int16",
      Value::Int32(_) => "int32",
      Value::Int64(_) => "int64",
      Value::Float32(_) => "float32",
      Value::Float64(_) => "float64",
      Value::String(_) => "string",
      Value::Array(_) => "array",
      Value::Object(_) => "object",
      Value::Bool(_) => "bool",
      Value::Null => "null",
      Value::Undefined => "undefined",
      Value::Marker(_) => "marker",
    }
  }
}
use std::fmt;
impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Int16(v) => write!(f, "{}", v),
      Value::Int32(v) => write!(f, "{}", v),
      Value::Int64(v) => write!(f, "{}", v),
      Value::Float32(v) => write!(f, "{}", v),
      Value::Float64(v) => write!(f, "{}", v),
      Value::Bool(v) => write!(f, "{}", v),
      Value::String(v) => write!(f, "{}", v),
      Value::Null => write!(f, "null"),
      Value::Undefined => write!(f, "undefined"),
      Value::Marker(v) => write!(f, "<Marker: {}>", v),
      Value::Array(v) => write!(f, "[Array({})]", v.len()),
      Value::Object(o) => write!(f, "[Object({})]", o.len()),
    }
  }
}
