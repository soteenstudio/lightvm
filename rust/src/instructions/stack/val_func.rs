use crate::types::value::Value;
use std::collections::HashMap;
pub fn val_func(vars: &mut HashMap<String, Value>, name: String) {
  vars
    .entry(name)
    .or_insert_with(|| Value::Marker("NoInitExpression".to_string()));
}
