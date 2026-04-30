use crate::types::instructions::Instructions;
use serde_json::Value as JsonValue;
pub fn run(bytecode_json: String) -> String {
  let raw_bytecode: Vec<JsonValue> = serde_json::from_str(&bytecode_json).expect("Invalid JSON");
  let bytecode: Vec<Instructions> = raw_bytecode
    .into_iter()
    .map(|item| Instructions::from_json_array(&item))
    .collect();
  let result = crate::vm::execute::execute(bytecode, None);
  serde_json::to_string(&result).unwrap()
}
