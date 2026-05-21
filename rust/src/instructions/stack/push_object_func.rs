use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use smol_str::SmolStr;
use std::sync::Arc;
#[inline(always)]
pub fn push_object_func(
  stack: &mut Vec<Value>,
  val: &Arc<AHashMap<SmolStr, Value>>,
  ip: usize,
) -> Result<(), VMError> {
  if stack.len() == stack.capacity() {
    return Err(VMError::StackOverflow {
      ip,
      limit: stack.capacity(),
    });
  }
  stack.push(Value::Object(val.clone()));
  Ok(())
}
