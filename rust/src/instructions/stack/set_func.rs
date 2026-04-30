use crate::types::value::Value;
use std::collections::HashMap;
pub fn set_func(stack: &mut Vec<Value>, vars: &mut HashMap<String, Value>, name: String) {
  if let Some(val) = stack.pop() {
    vars.insert(name, val);
    if stack.len() > 50 {
      stack.truncate(50);
    }
  } else {
    panic!("Stack underflow on SET");
  }
}
