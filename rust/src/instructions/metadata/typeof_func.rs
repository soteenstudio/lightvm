/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use ahash::AHashMap;
use smol_str::SmolStr;
pub fn typeof_func(stack: &mut Vec<Value>) -> Result<(), SmolStr> {
  let val = stack
    .pop()
    .ok_or_else(|| SmolStr::new("Stack underflow on TYPEOF"))?;
  let (class_name, is_nullable): (&str, bool) = match val {
    Value::Int32(_) => ("Integer", false),
    Value::Int64(_) => ("Long", false),
    Value::Float32(_) => ("Float", false),
    Value::Float64(_) => ("Double", false),
    Value::String(_) => ("String", false),
    Value::Bool(_) => ("Boolean", false),
    Value::Object(_) => ("Object", false),
    Value::Array(_) => ("Array", false),
    Value::Null => ("Null", true),
    Value::Undefined => ("Undefined", true),
    Value::Marker(_) => ("Marker", false),
  };
  let mut metadata = AHashMap::with_capacity(2);
  metadata.insert(
    SmolStr::new_static("type"),
    Value::String(SmolStr::new_static(class_name)),
  );
  metadata.insert(SmolStr::new_static("nullable"), Value::Bool(is_nullable));
  stack.push(Value::Object(metadata));
  Ok(())
}
