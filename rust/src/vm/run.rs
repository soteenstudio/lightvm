/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::types::instructions::Instructions;
use serde_json::Value as JsonValue;
#[inline]
#[cold]
pub fn run(bytecode_json: String) -> String {
  let raw_bytecode: Vec<JsonValue> = serde_json::from_str(&bytecode_json).expect("Invalid JSON");
  let bytecode: Vec<Instructions> = raw_bytecode
    .into_iter()
    .map(|item| Instructions::from_json_array(&item))
    .collect();
  let result = crate::vm::execute::execute(bytecode, None);
  match result {
    Ok(val) => serde_json::to_string(&val)
      .unwrap_or_else(|_| r#"{"error": "Failed to serialize result"}"#.to_string()),
    Err(err_msg) => {
      eprintln!("\n{}", err_msg);
      format!(r#"{{"status": "error", "message": {:?}}}"#, err_msg)
    }
  }
}
