use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use crate::utils::map_primitive::map_primitive;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
  pub fn from_json_array(item: &JsonValue) -> Self {
    let arr = item.as_array().expect("Instruction must be an array");
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
          Value::String(val.as_str().unwrap().to_string())
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
      "set" => Instructions::Set(arr[1].as_str().unwrap().to_string()),
      "get" => Instructions::Get(arr[1].as_str().unwrap().to_string()),
      "val" => Instructions::Val(arr[1].as_str().unwrap().to_string()),
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
      "func" => {
        let name = arr[1].as_str().unwrap().to_string();
        let params = arr[2].as_u64().unwrap() as u32;
        let start = arr[3].as_u64().unwrap() as usize;
        let end = arr[4].as_u64().unwrap() as usize;
        let mut names = Vec::new();
        for i in 5..arr.len() {
          names.push(arr[i].as_str().unwrap().to_string());
        }
        Instructions::Func(name, params, start, end, names)
      }
      "export" => Instructions::Export(arr[1].as_str().unwrap().to_string()),
      "return" => Instructions::Return,
      "call" => {
        let name = arr[1]
          .as_str()
          .expect("Function name must be a string")
          .to_string();
        let argc = arr[2].as_u64().expect("Argc must be a number") as u32;
        Instructions::Call(name, argc)
      }
      "stop" => Instructions::Stop,
      _ => panic!("Opcode '{}' belum di-map atau sampah!", op),
    }
  }
}
