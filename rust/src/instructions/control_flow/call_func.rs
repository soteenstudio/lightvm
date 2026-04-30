use crate::types::value::{FuncMetadata, Value};
use std::collections::HashMap;
pub fn call_func(
  name: &String,
  argc: u32,
  ip: &mut usize,
  stack: &mut Vec<Value>,
  call_stack: &mut Vec<usize>,
  vars: &mut HashMap<String, Value>,
  functions: &HashMap<String, FuncMetadata>,
) -> Result<(), String> {
  let fn_meta = functions
    .get(name)
    .ok_or_else(|| format!("Function {} not found", name))?;
  let mut args = Vec::new();
  for _ in 0..argc {
    args.push(stack.pop().ok_or("Stack underflow on CALL arguments")?);
  }
  args.reverse();
  call_stack.push(*ip);
  if stack.len() > 50 {
    stack.truncate(50);
  }
  for i in 0..fn_meta.params_count as usize {
    let param_name = fn_meta
      .param_names
      .get(i)
      .cloned()
      .unwrap_or_else(|| format!("param{}", i));
    let val = args.get(i).cloned().unwrap_or(Value::Undefined);
    vars.insert(param_name, val);
  }
  *ip = fn_meta.start;
  Ok(())
}
