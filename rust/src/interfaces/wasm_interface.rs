/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(feature = "wasm")]
use crate::interfaces::interface::LightVM;
use crate::types::{capability::Capability, vmconfig::VmWasmConfig};
use crate::utils::vmerror::VMError;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(js_name = "LightVM")]
pub struct WasmLightVM {
  inner: LightVM,
}
#[wasm_bindgen(js_class = "LightVM")]
impl WasmLightVM {
  #[wasm_bindgen(constructor)]
  pub fn new(config: JsValue) -> Result<WasmLightVM, JsValue> {
    let config: VmWasmConfig = serde_wasm_bindgen::from_value(config)
      .map_err(|e| js_sys::Error::new(&format!("Failed to parse config: {}", e)))?;
    let runtime_config = config.runtime_config.unwrap_or_default();
    let error_options = config.error_options.unwrap_or_default();
    use crate::types::value::Value;
    use crate::types::vmstate::VmState;
    use ahash::AHashMap;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    let mut caps_set = HashSet::new();
    if config.caps.is_empty() {
      caps_set.insert(Capability::Observe);
    } else {
      for cap_num in config.caps.iter() {
        match cap_num {
          0 => {
            caps_set.insert(Capability::Observe);
          }
          1 => {
            caps_set.insert(Capability::Control);
          }
          2 => {
            caps_set.insert(Capability::Debug);
          }
          3 => {
            caps_set.insert(Capability::Unsafe);
          }
          _ => {
            let vm_err = VMError::InvalidOpcode {
              ip: 0,
              code: smol_str::SmolStr::from(format!("UNKNOWN_CAPABILITY:{}", cap_num)),
            };
            return Err(wasm_bindgen::JsValue::from(js_sys::Error::new(
              &vm_err.to_string(),
            )));
          }
        }
      }
    }
    Ok(Self {
      inner: LightVM {
        bytecode: Vec::new(),
        listeners: AHashMap::new(),
        caps: caps_set,
        should_halt: Arc::new(AtomicBool::new(false)),
        state: VmState::Idle,
        _outputs: Vec::new(),
        _last_value: Value::Undefined,
        functions: AHashMap::new(),
        exported: HashSet::new(),
        _imports: AHashMap::new(),
        nightly: runtime_config.nightly.unwrap_or(false),
        backtrace: error_options.backtrace.unwrap_or(false),
        explain: error_options.explain.unwrap_or(false),
        hint: error_options.hint.unwrap_or(true),
      },
    })
  }
  #[wasm_bindgen(js_name = "withNightly")]
  pub fn with_nightly(&mut self, enabled: bool) {
    self.inner.nightly = enabled;
  }
  #[wasm_bindgen(js_name = "withBacktrace")]
  pub fn with_backtrace(&mut self, enabled: bool) {
    self.inner.backtrace = enabled;
  }
  #[wasm_bindgen(js_name = "withExplain")]
  pub fn with_explain(&mut self, enabled: bool) {
    self.inner.explain = enabled;
  }
  #[wasm_bindgen(js_name = "withHint")]
  pub fn with_hint(&mut self, enabled: bool) {
    self.inner.hint = enabled;
  }
  #[wasm_bindgen]
  pub fn load(&mut self, source: String) -> Result<(), JsValue> {
    self
      .inner
      .load_internal(source)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))
  }
  #[wasm_bindgen]
  pub fn run(&mut self) -> Result<(), JsValue> {
    self
      .inner
      .run_internal(None)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))
  }
  #[wasm_bindgen]
  pub fn provide(&mut self, name: String, value: JsValue) -> Result<(), JsValue> {
    let serde_val: serde_json::Value = serde_wasm_bindgen::from_value(value).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Invalid JS object for provide: {}",
        e
      )))
    })?;
    self
      .inner
      .provide_internal(name.into(), serde_val)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))
  }
  #[wasm_bindgen]
  pub fn inspect(&self) -> Result<JsValue, JsValue> {
    let json_str = self
      .inner
      .inspect_internal()
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    let serde_val: serde_json::Value = serde_json::from_str(&json_str).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Failed to parse inspect object: {}",
        e
      )))
    })?;
    serde_wasm_bindgen::to_value(&serde_val).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Wasm serialization failed: {}",
        e
      )))
    })
  }
  #[wasm_bindgen]
  pub fn halt(&mut self) -> Result<(), JsValue> {
    self
      .inner
      .halt_internal()
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))
  }
  #[wasm_bindgen]
  pub fn on(&mut self, event_type: String, callback: js_sys::Function) -> Result<(), JsValue> {
    use crate::types::vmevent::VmEvent;
    let event = match event_type.as_str() {
      "tick" => VmEvent::Tick,
      "halt" => VmEvent::Halt,
      "panic" => VmEvent::Panic,
      _ => {
        return Err(wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
          "Unknown event: {}",
          event_type
        ))));
      }
    };
    let js_func = RcFnWrapper::new(callback);
    self
      .inner
      .on_internal(event, move |payload| {
        let this = JsValue::null();
        let arg0 = JsValue::from_str(&payload);
        let _ = js_func.inner.call1(&this, &arg0);
      })
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e)))
  }
  #[wasm_bindgen]
  pub fn embedded(&mut self) -> Result<JsValue, JsValue> {
    self
      .inner
      .clear_outputs_internal()
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    self
      .inner
      .run_internal(None)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    let outputs = self
      .inner
      .get_outputs_internal()
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    let res_json = serde_json::json!({
      "value": serde_json::Value::Null,
      "outputs": outputs,
      "halted": true
    });
    serde_wasm_bindgen::to_value(&res_json).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Wasm serialization failed: {}",
        e
      )))
    })
  }
  #[wasm_bindgen(js_name = "callExport")]
  pub fn call_export(&mut self, name: String, args: JsValue) -> Result<JsValue, JsValue> {
    let serde_args: serde_json::Value = serde_wasm_bindgen::from_value(args).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!("Invalid export args: {}", e)))
    })?;
    let raw_result = self
      .inner
      .call_exported_internal(name, serde_args)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    let res_json: serde_json::Value = serde_json::from_str(&raw_result).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::from(format!(
        "Failed to parse export return value: {}",
        e
      )));
      wasm_bindgen::JsValue::from(js_sys::Error::new(&vm_err.to_string()))
    })?;
    serde_wasm_bindgen::to_value(&res_json).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Wasm serialization failed: {}",
        e
      )))
    })
  }
  #[wasm_bindgen]
  pub fn tools(&self) -> WasmLightVMTools {
    WasmLightVMTools {
      nightly: self.inner.nightly,
      backtrace: self.inner.backtrace,
      explain: self.inner.explain,
      hint: self.inner.hint,
    }
  }
}
#[wasm_bindgen(js_name = "LightVMTools")]
pub struct WasmLightVMTools {
  pub nightly: bool,
  pub backtrace: bool,
  pub explain: bool,
  pub hint: bool,
}
#[wasm_bindgen(js_class = "LightVMTools")]
impl WasmLightVMTools {
  #[wasm_bindgen(js_name = "optimizeBytecode")]
  pub fn optimize_bytecode(&self, bytecode: JsValue) -> Result<JsValue, JsValue> {
    use crate::utils::vmerror::VMError;
    let input_json: serde_json::Value = serde_wasm_bindgen::from_value(bytecode).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Invalid input structure: {}",
        e
      )))
    })?;
    let mut vm_instance = LightVM::new_node(self.nightly, self.backtrace, self.explain, self.hint);
    let opt_str = vm_instance
      .optimize_bytecode_internal(input_json)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    let res_json: serde_json::Value = serde_json::from_str(&opt_str).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::from(format!(
        "Internal JSON Parsing Failed: {}",
        e
      )));
      wasm_bindgen::JsValue::from(js_sys::Error::new(&vm_err.to_string()))
    })?;
    serde_wasm_bindgen::to_value(&res_json).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Wasm serialization failed: {}",
        e
      )))
    })
  }
  #[wasm_bindgen(js_name = "stringifyLtc")]
  pub fn stringify_ltc(&self, json: JsValue) -> Result<String, JsValue> {
    use crate::utils::vmerror::VMError;
    let serde_json: serde_json::Value = serde_wasm_bindgen::from_value(json).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Invalid json structure: {}",
        e
      )))
    })?;
    LightVM::stringify_ltc_internal(serde_json).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::from(e));
      wasm_bindgen::JsValue::from(js_sys::Error::new(&vm_err.to_string()))
    })
  }
  #[wasm_bindgen(js_name = "parseLtc")]
  pub fn parse_ltc(&self, code: String) -> Result<String, JsValue> {
    LightVM::parse_ltc_internal(code)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))
  }
  #[wasm_bindgen(js_name = "parseLtcArray")]
  pub fn parse_ltc_array(&self, code: String) -> JsValue {
    let res_json = LightVM::parse_ltc_array_internal(code);
    serde_wasm_bindgen::to_value(&res_json).unwrap_or(JsValue::null())
  }
}
struct RcFnWrapper {
  inner: js_sys::Function,
}
impl RcFnWrapper {
  fn new(inner: js_sys::Function) -> Self {
    Self { inner }
  }
}
unsafe impl Send for RcFnWrapper {}
unsafe impl Sync for RcFnWrapper {}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_config_parsing() {
    let json_data = serde_json::json!({
        "caps": [0, 2],
        "nightly": true,
        "backtrace": false,
        "explain": false,
        "hint": true
    });
    let config: VmWasmConfig = serde_json::from_value(json_data).unwrap();
    assert_eq!(config.caps, vec![0, 2]);
    vm.with_nightly(true);
    assert_eq!(vm.inner.nightly, true);
    vm.with_hint(false);
    assert_eq!(vm.inner.hint, false);
  }
}
