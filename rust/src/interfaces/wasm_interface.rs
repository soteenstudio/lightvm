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
use crate::types::capability::Capability;
use crate::utils::vmerror::VMError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "LightVM")]
pub struct WasmLightVM {
  inner: LightVM,
}

#[wasm_bindgen(js_class = "LightVM")]
impl WasmLightVM {
  #[wasm_bindgen(constructor)]
  pub fn new(caps_raw: Vec<u32>) -> Result<WasmLightVM, JsValue> {
    use crate::types::value::Value;
    use crate::types::vmstate::VmState;
    use ahash::AHashMap;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

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
            caps_set.insert(Capability::Debug);
          }
          3 => {
            caps_set.insert(Capability::Unsafe);
          }
          _ => {
            let vm_err = VMError::InvalidOpcode {
              ip: 0,
              code: smol_str::SmolStr::new(format!("UNKNOWN_CAPABILITY:{}", cap_num)),
            };
            return Err(js_sys::Error::new(&vm_err.to_string()).into());
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
      },
    })
  }

  #[wasm_bindgen]
  pub fn load(&mut self, source: String) -> Result<(), JsValue> {
    self
      .inner
      .load_internal(source)
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())
  }

  #[wasm_bindgen]
  pub fn run(&mut self) -> Result<(), JsValue> {
    self
      .inner
      .run_internal(None)
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())
  }

  #[wasm_bindgen]
  pub fn provide(&mut self, name: String, value: JsValue) -> Result<(), JsValue> {
    let serde_val: serde_json::Value = serde_wasm_bindgen::from_value(value)
      .map_err(|e| js_sys::Error::new(&format!("Invalid JS object for provide: {}", e)).into())?;

    self
      .inner
      .provide_internal(name.into(), serde_val)
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())
  }

  #[wasm_bindgen]
  pub fn inspect(&self) -> Result<JsValue, JsValue> {
    let json_str = self
      .inner
      .inspect_internal()
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())?;

    let serde_val: serde_json::Value = serde_json::from_str(&json_str)
      .map_err(|e| js_sys::Error::new(&format!("Failed to parse inspect object: {}", e)).into())?;

    serde_wasm_bindgen::to_value(&serde_val)
      .map_err(|e| js_sys::Error::new(&format!("Wasm serialization failed: {}", e)).into())
  }

  #[wasm_bindgen]
  pub fn halt(&mut self) -> Result<(), JsValue> {
    self
      .inner
      .halt_internal()
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())
  }

  #[wasm_bindgen]
  pub fn on(&mut self, event_type: String, callback: js_sys::Function) -> Result<(), JsValue> {
    use crate::types::vmevent::VmEvent;
    let event = match event_type.as_str() {
      "tick" => VmEvent::Tick,
      "halt" => VmEvent::Halt,
      "panic" => VmEvent::Panic,
      _ => return Err(js_sys::Error::new(&format!("Unknown event: {}", event_type)).into()),
    };

    // Di WASM / JS single-threaded environment, kita bisa langsung wrap js_sys::Function.
    // Ditambah Send + Sync static closure karena signature internal lo butuh trait tersebut.
    let js_func = RcFnWrapper::new(callback);

    self
      .inner
      .on_internal(event, move |payload| {
        let this = JsValue::null();
        let arg0 = JsValue::from_str(&payload);
        let _ = js_func.inner.call1(&this, &arg0);
      })
      .map_err(|e| js_sys::Error::new(&e).into())
  }

  #[wasm_bindgen]
  pub fn embedded(&mut self) -> Result<JsValue, JsValue> {
    self
      .inner
      .clear_outputs_internal()
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())?;

    self
      .inner
      .run_internal(None)
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())?;

    let outputs = self
      .inner
      .get_outputs_internal()
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())?;

    let res_json = serde_json::json!({
      "value": serde_json::Value::Null,
      "outputs": outputs,
      "halted": true
    });

    serde_wasm_bindgen::to_value(&res_json)
      .map_err(|e| js_sys::Error::new(&format!("Wasm serialization failed: {}", e)).into())
  }

  #[wasm_bindgen(js_name = "callExport")]
  pub fn call_export(&mut self, name: String, args: JsValue) -> Result<JsValue, JsValue> {
    let serde_args: serde_json::Value = serde_wasm_bindgen::from_value(args)
      .map_err(|e| js_sys::Error::new(&format!("Invalid export args: {}", e)).into())?;

    let raw_result = self
      .inner
      .call_exported_internal(name, serde_args)
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())?;

    let res_json: serde_json::Value = serde_json::from_str(&raw_result).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Failed to parse export return value: {}",
        e
      )));
      js_sys::Error::new(&vm_err.to_string()).into()
    })?;

    serde_wasm_bindgen::to_value(&res_json)
      .map_err(|e| js_sys::Error::new(&format!("Wasm serialization failed: {}", e)).into())
  }

  #[wasm_bindgen(js_name = "optimizeBytecode")]
  pub fn optimize_bytecode(bytecode: JsValue) -> Result<JsValue, JsValue> {
    let input_json: serde_json::Value = serde_wasm_bindgen::from_value(bytecode)
      .map_err(|e| js_sys::Error::new(&format!("Invalid input structure: {}", e)).into())?;

    let opt_str = LightVM::optimize_bytecode_internal(input_json)
      .map_err(|e| js_sys::Error::new(&e.to_string()).into())?;

    let res_json: serde_json::Value = serde_json::from_str(&opt_str).map_err(|e| {
      let vm_err = VMError::SystemError(smol_str::SmolStr::new(format!(
        "Internal JSON Parsing Failed: {}",
        e
      )));
      js_sys::Error::new(&vm_err.to_string()).into()
    })?;

    serde_wasm_bindgen::to_value(&res_json)
      .map_err(|e| js_sys::Error::new(&format!("Wasm serialization failed: {}", e)).into())
  }

  #[wasm_bindgen(js_name = "stringifyLtc")]
  pub fn stringify_ltc(json: JsValue) -> Result<String, JsValue> {
    let serde_json: serde_json::Value = serde_wasm_bindgen::from_value(json)
      .map_err(|e| js_sys::Error::new(&format!("Invalid json structure: {}", e)).into())?;

    LightVM::stringify_ltc_internal(serde_json).map_err(|e| js_sys::Error::new(&e).into())
  }

  #[wasm_bindgen(js_name = "parseLtc")]
  pub fn parse_ltc(code: String) -> Result<String, JsValue> {
    LightVM::parse_ltc_internal(code).map_err(|e| js_sys::Error::new(&e.to_string()).into())
  }

  #[wasm_bindgen(js_name = "parseLtcArray")]
  pub fn parse_ltc_array(code: String) -> JsValue {
    let res_json = LightVM::parse_ltc_array_internal(code);
    serde_wasm_bindgen::to_value(&res_json).unwrap_or(JsValue::null())
  }
}

// Helper struct biar js_sys::Function aman dibawa muter-muter lewat closure internal LightVM
// yang nge-require bound Send + Sync (Wasm di browser/Node environment single threaded aman pakai unsafe bypass ini).
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
