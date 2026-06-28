/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use crate::types::value::RunOptions;
use crate::utils::vmerror::VMError;
use serde_json::Value as JsonValue;
pub fn execute_and_log(bytecode: Vec<Instructions>, options: Option<RunOptions>) -> String {
  let halt_flag = options.as_ref().map(|o| o.halt_flag.clone());
  let result = crate::vm::execute::execute(bytecode, options, halt_flag);
  match result {
    Ok(val) => serde_json::to_string(&val).unwrap_or_default(),
    Err(err) => {
      eprintln!("\n{}", err);
      format!(r#"{{"status": "error", "message": {:?}}}"#, err)
    }
  }
}
#[inline]
#[cold]
pub fn run(bytecode_json: &str, options: Option<RunOptions>) -> String {
  let raw: Vec<JsonValue> = serde_json::from_str(bytecode_json).expect("Invalid JSON");
  let bytecode_res: Result<Vec<Instructions>, VMError> =
    raw.iter().map(Instructions::from_json_array).collect();
  match bytecode_res {
    Ok(bytecode) => execute_and_log(bytecode, options),
    Err(err) => {
      eprintln!("\n{}", err);
      format!(r#"{{"status": "error", "message": {:?}}}"#, err)
    }
  }
}
