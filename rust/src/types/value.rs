use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
  Int32(i32),
  Int64(i64),
  Float32(f32),
  Float64(f64),
  String(String),
  Bool(bool),
  Null,
  Undefined,
  Marker(String),
}
#[derive(Clone)]
pub struct FuncMetadata {
  pub params_count: u32,
  pub param_names: Vec<String>,
  pub start: usize,
  pub end: usize,
}
pub struct RunOptions {
  pub entry: Option<usize>,
  pub args: Vec<Value>,
  pub capture_return: bool,
}
impl Value {
  pub fn as_i32(&self) -> i32 {
    match self {
      Value::Int32(v) => *v,
      Value::Int64(v) => *v as i32,
      Value::Float64(v) => *v as i32,
      _ => panic!("Expected Int32 compatible value, found {:?}", self),
    }
  }
  pub fn as_i64(&self) -> i64 {
    match self {
      Value::Int64(v) => *v,
      Value::Int32(v) => *v as i64,
      Value::Float64(v) => *v as i64,
      _ => panic!("Expected Int64 compatible value, found {:?}", self),
    }
  }
  pub fn as_f32(&self) -> f32 {
    match self {
      Value::Float32(v) => *v,
      Value::Float64(v) => *v as f32,
      Value::Int32(v) => *v as f32,
      Value::Int64(v) => *v as f32,
      _ => panic!("Expected Float32 compatible value, found {:?}", self),
    }
  }
  pub fn as_f64(&self) -> f64 {
    match self {
      Value::Float64(v) => *v,
      Value::Float32(v) => *v as f64,
      Value::Int32(v) => *v as f64,
      Value::Int64(v) => *v as f64,
      _ => panic!("Expected Float64 compatible value, found {:?}", self),
    }
  }
  pub fn as_string(&self) -> String {
    match self {
      Value::String(v) => v.clone(),
      Value::Int32(v) => v.to_string(),
      Value::Int64(v) => v.to_string(),
      Value::Float32(v) => v.to_string(),
      Value::Float64(v) => v.to_string(),
      Value::Bool(v) => v.to_string(),
      Value::Null => "null".to_string(),
      Value::Undefined => "undefined".to_string(),
      Value::Marker(v) => v.clone(),
    }
  }
  pub fn as_bool(&self) -> bool {
    match self {
      Value::Bool(v) => *v,
      Value::Int32(v) => *v != 0,
      Value::Int64(v) => *v != 0,
      Value::Float32(v) => *v != 0.0,
      Value::Float64(v) => *v != 0.0,
      Value::String(v) => !v.is_empty(),
      Value::Null => false,
      Value::Undefined => false,
      Value::Marker(_) => true,
    }
  }
  pub fn type_of(&self) -> &'static str {
    match self {
      Value::Int32(_) => "int32",
      Value::Int64(_) => "int64",
      Value::Float32(_) => "float32",
      Value::Float64(_) => "float64",
      Value::String(_) => "string",
      Value::Bool(_) => "bool",
      Value::Null => "null",
      Value::Undefined => "undefined",
      Value::Marker(_) => "marker",
    }
  }
}
