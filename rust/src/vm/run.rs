/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::vmerror::VMError;
use crate::types::instructions::Instructions;
use crate::types::value::{RunOptions, Value};
use serde::Serialize;
use serde_json::Value as JsonValue;
#[derive(Serialize)]
struct ValuePayload {
  defined: bool,
  value: Value,
}
pub fn execute_and_log(
  bytecode: Vec<Instructions>,
  options: &mut Option<RunOptions>,
) -> Result<String, VMError> {
  let halt_flag = options.as_ref().map(|o| o.halt_flag.clone());
  let (val, tick) = crate::vm::execute::execute(bytecode, options, halt_flag)?;
  let defined = !matches!(val, Value::Undefined);
  let payload = ValuePayload {
    defined,
    value: val,
  };
  Ok(
    serde_json::json!({
        "status": "success",
        "result": payload,
        "ticks": tick
    })
    .to_string(),
  )
}
#[inline]
#[cold]
pub fn run(bytecode_json: &str, options: &mut Option<RunOptions>) -> Result<String, VMError> {
  let raw: Vec<JsonValue> = serde_json::from_str(bytecode_json).map_err(|e| {
    VMError::SystemError(smol_str::SmolStr::new(format!(
      "Failed to parse JSON: {}",
      e
    )))
  })?;
  let bytecode: Vec<Instructions> = raw
    .iter()
    .enumerate()
    .map(|(ip, item)| Instructions::from_json_array(item, ip))
    .collect::<Result<Vec<Instructions>, VMError>>()?;
  execute_and_log(bytecode, options)
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_run_with_valid_json() {
    let json = r#"[["push", 10], ["stop"]]"#;
    let result = run(json, &mut None);
    assert!(result.is_ok());
    let result_str = result.unwrap();
    assert!(result_str.contains("10") || result_str.contains("status"));
  }
  #[test]
  fn test_run_with_invalid_instruction() {
    let json = r#"[["random nonsense", 0]]"#;
    let result = run(json, &mut None);
    assert!(result.is_err());
  }
  #[test]
  fn test_run_with_malformed_json() {
    let json = r#"not valid json at all"#;
    let result = run(json, &mut None);
    assert!(result.is_err());
    if let Err(VMError::SystemError(msg)) = result {
      assert!(msg.contains("Failed to parse JSON"));
    } else {
      panic!("Expected SystemError with JSON parse message");
    }
  }
}
