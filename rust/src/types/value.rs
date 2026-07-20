/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::fast_format::{float_to_smol, int_to_smol};
use ahash::AHashMap;
use half::f16;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use ts_rs::TS;
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[serde(untagged)]
#[ts(export)]
pub enum Value {
  Int16(i16),
  Int32(i32),
  Int64(i64),
  #[ts(type = "number")]
  Int128(i128),
  #[ts(type = "number")]
  Float16(f16),
  Float32(f32),
  Float64(f64),
  #[ts(type = "string")]
  String(SmolStr),
  #[ts(type = "any[]")]
  Array(Arc<Vec<Value>>),
  #[ts(type = "Record<string, any>")]
  Object(Arc<AHashMap<SmolStr, Value>>),
  Bool(bool),
  Null,
  #[default]
  Undefined,
  NaN,
  #[ts(type = "string")]
  Marker(SmolStr),
}
#[derive(Clone, Debug)]
pub struct FuncMetadata {
  pub params_count: u32,
  pub param_names: Vec<SmolStr>,
  pub start: usize,
  pub end: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunOptions {
  pub entry: Option<usize>,
  pub args: Vec<Value>,
  pub capture_return: bool,
  pub imports: AHashMap<SmolStr, Value>,
  pub halt_flag: Arc<AtomicBool>,
}
impl Value {
  #[inline(always)]
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Bool(v) => *v,
      Value::Int16(v) => *v != 0,
      Value::Int32(v) => *v != 0,
      Value::Int64(v) => *v != 0,
      Value::Int128(v) => *v != 0,
      Value::Float16(v) => *v != half::f16::ZERO && !v.is_nan(),
      Value::Float32(v) => *v != 0.0 && !v.is_nan(),
      Value::Float64(v) => *v != 0.0 && !v.is_nan(),
      Value::String(v) => !v.is_empty(),
      Value::Null | Value::Undefined | Value::NaN => false,
      Value::Marker(_) => true,
      Value::Array(_) | Value::Object(_) => true,
    }
  }
  #[inline(always)]
  pub fn is_number(&self) -> bool {
    matches!(
      self,
      Value::Int16(_)
        | Value::Int32(_)
        | Value::Int64(_)
        | Value::Float16(_)
        | Value::Float32(_)
        | Value::Float64(_)
    )
  }
  #[inline(always)]
  pub fn as_i16(&self) -> i16 {
    match self {
      Value::Int16(v) => *v,
      Value::Int32(v) => *v as i16,
      Value::Int64(v) => *v as i16,
      Value::Int128(v) => *v as i16,
      Value::Float16(v) => v.to_f32() as i16,
      Value::Float32(v) => *v as i16,
      Value::Float64(v) => *v as i16,
      _ => 0,
    }
  }
  #[inline(always)]
  pub fn as_i32(&self) -> i32 {
    match self {
      Value::Int32(v) => *v,
      Value::Int16(v) => *v as i32,
      Value::Int64(v) => *v as i32,
      Value::Int128(v) => *v as i32,
      Value::Float16(v) => v.to_f32() as i32,
      Value::Float32(v) => *v as i32,
      Value::Float64(v) => *v as i32,
      _ => 0,
    }
  }
  #[inline(always)]
  pub fn as_i64(&self) -> i64 {
    match self {
      Value::Int64(v) => *v,
      Value::Int16(v) => *v as i64,
      Value::Int32(v) => *v as i64,
      Value::Int128(v) => *v as i64,
      Value::Float16(v) => v.to_f64() as i64,
      Value::Float32(v) => *v as i64,
      Value::Float64(v) => *v as i64,
      _ => 0,
    }
  }
  #[inline(always)]
  pub fn as_i128(&self) -> i128 {
    match self {
      Value::Int128(v) => *v,
      Value::Int16(v) => *v as i128,
      Value::Int32(v) => *v as i128,
      Value::Int64(v) => *v as i128,
      Value::Float16(v) => v.to_f64() as i128,
      Value::Float32(v) => *v as i128,
      Value::Float64(v) => *v as i128,
      _ => 0,
    }
  }
  #[inline(always)]
  pub fn as_f16(&self) -> f16 {
    match self {
      Value::Float16(v) => *v,
      Value::Float32(v) => f16::from_f32(*v),
      Value::Float64(v) => f16::from_f64(*v),
      Value::Int16(v) => f16::from_f32(*v as f32),
      Value::Int32(v) => f16::from_f32(*v as f32),
      Value::Int64(v) => f16::from_f32(*v as f32),
      Value::Int128(v) => f16::from_f32(*v as f32),
      _ => f16::ZERO,
    }
  }
  #[inline(always)]
  pub fn as_f32(&self) -> f32 {
    match self {
      Value::Float32(v) => *v,
      Value::Float16(v) => v.to_f32(),
      Value::Float64(v) => *v as f32,
      Value::Int16(v) => *v as f32,
      Value::Int32(v) => *v as f32,
      Value::Int64(v) => *v as f32,
      Value::Int128(v) => *v as f32,
      _ => 0.0,
    }
  }
  #[inline(always)]
  pub fn as_f64(&self) -> f64 {
    match self {
      Value::Float64(v) => *v,
      Value::Float16(v) => v.to_f64(),
      Value::Float32(v) => *v as f64,
      Value::Int16(v) => *v as f64,
      Value::Int32(v) => *v as f64,
      Value::Int64(v) => *v as f64,
      Value::Int128(v) => *v as f64,
      _ => 0.0,
    }
  }
  #[inline]
  pub fn as_string(&self) -> SmolStr {
    match self {
      Value::String(v) => v.clone(),
      Value::Int16(v) => int_to_smol(*v as i64),
      Value::Int32(v) => int_to_smol(*v as i64),
      Value::Int64(v) => int_to_smol(*v),
      Value::Int128(v) => int_to_smol(*v as i64),
      Value::Float16(v) => float_to_smol(v.to_f64()),
      Value::Float32(v) => float_to_smol(*v as f64),
      Value::Float64(v) => float_to_smol(*v),
      Value::Bool(v) => SmolStr::new(if *v { "true" } else { "false" }),
      Value::Null => SmolStr::new("null"),
      Value::Undefined => SmolStr::new("undefined"),
      Value::Marker(v) => v.clone(),
      Value::Array(_) => SmolStr::new_static("[array]"),
      Value::NaN => SmolStr::new_static("NaN"),
      Value::Object(_) => SmolStr::new_static("[object]"),
    }
  }
  pub fn type_of(&self) -> &'static str {
    match self {
      Value::Int16(_) => "int16",
      Value::Int32(_) => "int32",
      Value::Int64(_) => "int64",
      Value::Int128(_) => "int128",
      Value::Float16(_) => "float16",
      Value::Float32(_) => "float32",
      Value::Float64(_) => "float64",
      Value::String(_) => "string",
      Value::Array(_) => "array",
      Value::Object(_) => "object",
      Value::Bool(_) => "bool",
      Value::Null => "null",
      Value::Undefined => "undefined",
      Value::NaN => "nan",
      Value::Marker(_) => "marker",
    }
  }
}
impl From<serde_json::Value> for Value {
  fn from(json: serde_json::Value) -> Self {
    match json {
      serde_json::Value::String(s) => Value::String(SmolStr::from(s)),
      serde_json::Value::Number(n) => {
        if let Some(i) = n.as_i64() {
          Value::Int64(i)
        } else if let Some(f) = n.as_f64() {
          Value::Float64(f)
        } else {
          Value::NaN
        }
      }
      serde_json::Value::Bool(b) => Value::Bool(b),
      serde_json::Value::Null => Value::Null,
      serde_json::Value::Array(a) => {
        let converted: Vec<Value> = a.into_iter().map(Value::from).collect();
        Value::Array(Arc::new(converted))
      }
      _ => Value::Undefined,
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
      Value::Int128(v) => write!(f, "{}", v),
      Value::Float16(v) => write!(f, "{}", v),
      Value::Float32(v) => write!(f, "{}", v),
      Value::Float64(v) => write!(f, "{}", v),
      Value::Bool(v) => write!(f, "{}", v),
      Value::String(v) => write!(f, "{}", v),
      Value::Null => write!(f, "null"),
      Value::Undefined => write!(f, "undefined"),
      Value::NaN => write!(f, "NaN"),
      Value::Marker(v) => write!(f, "<Marker: {}>", v),
      Value::Array(v) => write!(f, "[Array({})]", v.len()),
      Value::Object(o) => write!(f, "[Object({})]", o.len()),
    }
  }
}
