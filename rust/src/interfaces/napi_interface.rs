/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(feature = "node")]
use crate::interfaces::interface::LightVM;
use crate::types::capability::Capability;
use crate::utils::vmerror::VMError;
use napi::bindgen_prelude::*;
use napi_derive::napi;
#[napi(js_name = "LightVM")]
pub struct NodeLightVM {
  inner: LightVM,
}
#[napi]
impl NodeLightVM {
  #[napi(constructor)]
  pub fn napi_new(caps_raw: Vec<u32>) -> Result<Self> {
    use crate::types::value::Value;
    use crate::types::vmstate::VmState;
    use ahash::AHashMap;
    use std::collections::HashSet;
    let mut caps_set = HashSet::new();
    if caps_raw.is_empty() {
      caps_set.insert(Capability::Observe);
    } else {
      for cap_num in caps_raw {
        match cap_num {
          0 => {
            caps_set.insert(Capability::Observe);
          }
          1 => {
            caps_set.insert(Capability::Control);
          }
          2 => {
            caps_set.insert(Capability::Unsafe);
          }
          _ => {
            let vm_err = VMError::InvalidOpcode {
              ip: 0,
              code: smol_str::SmolStr::new(format!("UNKNOWN_CAPABILITY:{}", cap_num)),
            };
            return Err(Error::from_reason(vm_err.format().to_string()));
          }
        }
      }
    }
    Ok(Self {
      inner: LightVM {
        bytecode: Vec::new(),
        listeners: AHashMap::new(),
        caps: caps_set,
        state: VmState::Idle,
        _outputs: Vec::new(),
        _last_value: Value::Undefined,
        functions: AHashMap::new(),
        exported: HashSet::new(),
        _imports: AHashMap::new(),
      },
    })
  }
  #[napi]
  pub fn load(&mut self, source: String) -> Result<()> {
    self
      .inner
      .load_internal(source)
      .map_err(|e| Error::from_reason(e.format().to_string()))
  }
  #[napi]
  pub fn run(&mut self) -> Result<()> {
    self
      .inner
      .run_internal(None)
      .map_err(|e| Error::from_reason(e.format().to_string()))
  }
  #[napi]
  pub fn provide(&mut self, name: String, value: serde_json::Value) -> Result<()> {
    self
      .inner
      .provide_internal(name.into(), value)
      .map_err(|e| Error::from_reason(e.format().to_string()))
  }
  #[napi]
  pub fn inspect(&self) -> Result<serde_json::Value> {
    use crate::utils::vmerror::VMError;
    let json_str = self
      .inner
      .inspect_internal()
      .map_err(|e| Error::from_reason(e.format().to_string()))?;
    serde_json::from_str(&json_str).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Failed to parse inspect object: {}",
        e
      )));
      Error::from_reason(vm_err.format().to_string())
    })
  }
  #[napi]
  pub fn halt(&mut self) -> Result<()> {
    self
      .inner
      .halt_internal()
      .map_err(|e| Error::from_reason(e.format().to_string()))
  }
  #[napi]
  pub fn embedded(&mut self) -> Result<serde_json::Value> {
    self
      .inner
      .clear_outputs_internal()
      .map_err(|e| Error::from_reason(e.format().to_string()))?;
    self
      .inner
      .run_internal(None)
      .map_err(|e| Error::from_reason(e.format().to_string()))?;
    let outputs = self
      .inner
      .get_outputs_internal()
      .map_err(|e| Error::from_reason(e.format().to_string()))?;
    Ok(serde_json::json!({
      "value": serde_json::Value::Null,
      "outputs": outputs,
      "halted": true
    }))
  }
  #[napi(js_name = "callExport")]
  pub fn call_export(
    &mut self,
    name: String,
    args: serde_json::Value,
  ) -> Result<serde_json::Value> {
    let raw_result = self
      .inner
      .call_exported_internal(name, args)
      .map_err(|e| Error::from_reason(e.format().to_string()))?;
    serde_json::from_str(&raw_result).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Failed to parse export return value: {}",
        e
      )));
      Error::from_reason(vm_err.format().to_string())
    })
  }
  #[napi(js_name = "optimizeBytecode")]
  pub fn napi_optimize_bytecode(bytecode: serde_json::Value) -> Result<serde_json::Value> {
    use crate::utils::vmerror::VMError;
    let input_string = serde_json::to_string(&bytecode).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Failed to serialize input: {}",
        e
      )));
      Error::from_reason(vm_err.format().to_string())
    })?;
    let input_json: serde_json::Value = serde_json::from_str(&input_string).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Invalid input structure: {}",
        e
      )));
      Error::from_reason(vm_err.format().to_string())
    })?;
    let opt_str = LightVM::optimize_bytecode_internal(input_json)
      .map_err(|e| Error::from_reason(e.format().to_string()))?;
    serde_json::from_str::<serde_json::Value>(&opt_str).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Internal JSON Parsing Failed: {}",
        e
      )));
      Error::from_reason(vm_err.format().to_string())
    })
  }
  #[napi(js_name = "stringifyLtc")]
  pub fn napi_stringify_ltc(json: serde_json::Value) -> Result<String> {
    LightVM::stringify_ltc_internal(json).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(e));
      Error::from_reason(vm_err.format().to_string())
    })
  }
  #[napi(js_name = "parseLtc")]
  pub fn napi_parse_ltc(code: String) -> Result<String> {
    LightVM::parse_ltc_internal(code).map_err(|e| Error::from_reason(e.format().to_string()))
  }
  #[napi(js_name = "parseLtcArray")]
  pub fn napi_parse_ltc_array(code: String) -> serde_json::Value {
    LightVM::parse_ltc_array_internal(code)
  }
}
