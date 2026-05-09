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
use std::sync::Arc;
pub fn typeof_func(stack: &mut Vec<Value>) -> Result<(), SmolStr> {
  if let Some(top) = stack.last_mut() {
    let (class_name, is_nullable) = match top {
      Value::Int16(_) => ("Short", false),
      Value::Int32(_) => ("Integer", false),
      Value::Int64(_) => ("Long", false),
      Value::Float16(_) => ("Half", false),
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
    *top = Value::Object(Arc::new(metadata));
    Ok(())
  } else {
    Err(SmolStr::new_static("Stack underflow on TYPEOF"))
  }
}
