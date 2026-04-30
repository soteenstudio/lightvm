use crate::types::value::Value;
use std::collections::HashMap;
pub fn get_func(stack: &mut Vec<Value>, vars: &HashMap<String, Value>, name: String) {
  let val = vars.get(&name).cloned().unwrap_or(Value::Undefined);
  stack.push(val);
}
