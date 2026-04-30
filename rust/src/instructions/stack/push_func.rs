use crate::types::value::Value;
pub fn push_func(stack: &mut Vec<Value>, val: Value) {
  stack.push(val);
}
