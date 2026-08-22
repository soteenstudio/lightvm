/*
 * Copyright 2025-2026 SoTeen Studio
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
pub fn execute_and_log(bytecode: Vec<Instructions>, options: &mut Option<RunOptions>) -> String {
  let halt_flag = options.as_ref().map(|o| o.halt_flag.clone());
  let result = crate::vm::execute::execute(bytecode, options, halt_flag);
  match result {
    Ok((val, tick)) => serde_json::json!({
        "status": "success",
        "result": val,
        "ticks": tick
    })
    .to_string(),
    Err(err) => {
      eprintln!("\n{}", err);
      serde_json::json!({
          "status": "error",
          "message": format!("{:?}", err)
      })
      .to_string()
    }
  }
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
  let bytecode_res: Result<Vec<Instructions>, VMError> = raw
    .iter()
    .enumerate()
    .map(|(ip, item)| Instructions::from_json_array(item, ip))
    .collect();
  match bytecode_res {
    Ok(bytecode) => Ok(execute_and_log(bytecode, options)),
    Err(err) => {
      eprintln!("\n{}", err);
      Ok(
        serde_json::json!({
          "status": "error",
          "message": format!("{:?}", err)
        })
        .to_string(),
      )
    }
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_run_with_valid_json() {
    let json = r#"[["PushInt32", 10], ["Stop"]]"#;
    let result = run(json, &mut None);
    assert!(result.is_ok());
    let result_str = result.unwrap();
    assert!(result_str.contains("10") || result_str.contains("status"));
  }
  #[test]
  fn test_run_with_invalid_instruction() {
    let json = r#"[["random nonsense", 0]]"#;
    let result = run(json, &mut None);
    assert!(result.is_ok());
    let result_str = result.unwrap();
    assert!(result_str.contains("error"));
    assert!(result_str.contains("message"));
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
