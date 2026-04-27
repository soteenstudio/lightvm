use crate::types::instructions::Instructions;
use crate::types::value::Value;
use crate::utils::map_primitive::map_primitive;
use serde_json::Value as JsonValue;
pub fn run(bytecode_json: String) -> String {
  let raw_bytecode: Vec<JsonValue> = serde_json::from_str(&bytecode_json).expect("Invalid JSON");
  let bytecode: Vec<Instructions> = raw_bytecode
    .into_iter()
    .map(|item| {
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
    })
    .collect();
  let result = crate::vm::execute::execute(bytecode, None);
  serde_json::to_string(&result).unwrap()
}