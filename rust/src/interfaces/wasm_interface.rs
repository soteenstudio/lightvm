/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(feature = "wasm")]
use crate::interfaces::interface::LightVM;
use crate::modules::vmerror::VMError;
use crate::types::{capability::Capability, time_budget::TimeBudget, vmconfig::VmWasmConfig};
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
            caps_set.insert(Capability::Control);
          }
          1 => {
            caps_set.insert(Capability::Observe);
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
    use crate::types::security_config::SecurityConfig;
    let default_security = SecurityConfig::default();
    Ok(Self {
      inner: LightVM {
        bytecode: Vec::new(),
        listeners: AHashMap::new(),
        next_listener_id: 0,
        caps: caps_set,
        should_halt: Arc::new(AtomicBool::new(false)),
        state: VmState::Idle,
        _outputs: Vec::new(),
        _last_value: Value::Undefined,
        functions: AHashMap::new(),
        exported: HashSet::new(),
        _imports: AHashMap::new(),
        last_run_options: None,
        max_io: default_security.max_io,
        max_import: default_security.max_import,
        max_alloc: default_security.max_alloc,
        max_call: default_security.max_call,
        max_jump: default_security.max_jump,
        max_ticks: default_security.max_ticks,
        max_stack_size: default_security.max_stack_size,
        allowed_imports: default_security.allowed_imports,
        unsafe_mode: default_security.unsafe_mode,
        time_budget: default_security.time_budget,
        nightly: runtime_config.nightly.unwrap_or(false),
        backtrace: error_options.backtrace.unwrap_or(false),
        explain: error_options.explain.unwrap_or(false),
        hint: error_options.hint.unwrap_or(true),
      },
    })
  }
  #[wasm_bindgen(js_name = "setMaxIo")]
  pub fn set_max_io(&mut self, value: usize) {
    self.inner.max_io = value;
  }
  #[wasm_bindgen(js_name = "setMaxImport")]
  pub fn set_max_import(&mut self, value: usize) {
    self.inner.max_import = value;
  }
  #[wasm_bindgen(js_name = "setMaxAlloc")]
  pub fn set_max_alloc(&mut self, value: usize) {
    self.inner.max_alloc = value;
  }
  #[wasm_bindgen(js_name = "setMaxCall")]
  pub fn set_max_call(&mut self, value: usize) {
    self.inner.max_call = value;
  }
  #[wasm_bindgen(js_name = "setMaxJump")]
  pub fn set_max_jump(&mut self, value: usize) {
    self.inner.max_jump = value;
  }
  #[wasm_bindgen(js_name = "setMaxTicks")]
  pub fn set_max_ticks(&mut self, value: f64) {
    self.inner.max_ticks = value as u64;
  }
  #[wasm_bindgen(js_name = "setMaxStackSize")]
  pub fn set_max_stack_size(&mut self, value: usize) {
    self.inner.max_stack_size = value;
  }
  #[wasm_bindgen(js_name = "setAllowedImports")]
  pub fn set_allowed_imports(&mut self, value: Vec<String>) {
    self.inner.allowed_imports = value;
  }
  #[wasm_bindgen(js_name = "setTimeBudget")]
  pub fn set_time_budget(&mut self, value: u32) -> Result<(), JsValue> {
    let budget = match value {
      0 => TimeBudget::Cheap,
      1 => TimeBudget::Normal,
      2 => TimeBudget::Expensive,
      _ => {
        return Err(JsValue::from(js_sys::Error::new(&format!(
          "Unknown time budget: {}",
          value
        ))));
      }
    };
    self.inner.time_budget = budget;
    Ok(())
  }
  #[wasm_bindgen(js_name = "withUnsafeMode")]
  pub fn with_unsafe_mode(&mut self, enabled: bool) {
    self.inner.unsafe_mode = enabled;
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
  pub fn run(&mut self) -> Result<JsValue, JsValue> {
    let raw_json = self
      .inner
      .run_internal(None)
      .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
    let parsed: serde_json::Value = serde_json::from_str(&raw_json).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Failed to parse VM result: {}",
        e
      )))
    })?;
    serde_wasm_bindgen::to_value(&parsed).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Wasm serialization failed: {}",
        e
      )))
    })
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
    use crate::interfaces::interface::VmEventData;
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
      .on_internal(event, move |data: &VmEventData| {
        let this = JsValue::null();
        let arg0 = serde_wasm_bindgen::to_value(&serde_json::json!({
          "event": data.event,
          "payload": data.payload,
        }))
        .unwrap_or(JsValue::NULL);
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
    let target_name = name.clone();
    let is_function = self.inner.functions.contains_key(target_name.as_str());
    let serde_args: serde_json::Value = serde_wasm_bindgen::from_value(args).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!("Invalid export args: {}", e)))
    })?;
    if is_function {
      let raw_result = self
        .inner
        .call_exported_internal(target_name, serde_args)
        .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
      let parsed: serde_json::Value = serde_json::from_str(&raw_result).map_err(|e| {
        let vm_err = VMError::SystemError(smol_str::SmolStr::from(format!(
          "Failed to parse export return value: {}",
          e
        )));
        wasm_bindgen::JsValue::from(js_sys::Error::new(&vm_err.to_string()))
      })?;
      if parsed["status"] == "success" {
        let result_payload = parsed
          .get("result")
          .cloned()
          .unwrap_or(serde_json::Value::Null);
        let unwrapped_value = if let Some(obj) = result_payload.as_object() {
          let defined = obj.get("defined").and_then(|v| v.as_bool()).unwrap_or(true);
          if defined {
            obj.get("value").cloned().unwrap_or(serde_json::Value::Null)
          } else {
            serde_json::Value::Null
          }
        } else {
          result_payload
        };
        serde_wasm_bindgen::to_value(&unwrapped_value).map_err(|e| {
          wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
            "Wasm serialization failed: {}",
            e
          )))
        })
      } else {
        Err(wasm_bindgen::JsValue::from(js_sys::Error::new(
          parsed["message"].as_str().unwrap_or("Unknown Error"),
        )))
      }
    } else {
      let raw_result = self
        .inner
        .var_exported_internal(target_name)
        .map_err(|e| wasm_bindgen::JsValue::from(js_sys::Error::new(&e.to_string())))?;
      let parsed: serde_json::Value = serde_json::from_str(&raw_result).map_err(|e| {
        let vm_err = VMError::SystemError(smol_str::SmolStr::from(format!(
          "Failed to parse variable: {}",
          e
        )));
        wasm_bindgen::JsValue::from(js_sys::Error::new(&vm_err.to_string()))
      })?;
      let unwrapped_value = if let Some(obj) = parsed.as_object() {
        let defined = obj.get("defined").and_then(|v| v.as_bool()).unwrap_or(true);
        if defined {
          obj.get("value").cloned().unwrap_or(serde_json::Value::Null)
        } else {
          serde_json::Value::Null
        }
      } else {
        parsed
      };
      serde_wasm_bindgen::to_value(&unwrapped_value).map_err(|e| {
        wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
          "Wasm serialization failed: {}",
          e
        )))
      })
    }
  }
  #[wasm_bindgen]
  pub fn tools(&self) -> WasmLightVMTools {
    WasmLightVMTools {
      nightly: self.inner.nightly,
      backtrace: self.inner.backtrace,
      explain: self.inner.explain,
      hint: self.inner.hint,
      time_budget: self.inner.time_budget,
      can_observe: self.inner.caps.contains(&Capability::Observe),
      can_control: self.inner.caps.contains(&Capability::Control),
      can_debug: self.inner.caps.contains(&Capability::Debug),
      can_unsafe: self.inner.caps.contains(&Capability::Unsafe),
    }
  }
}
#[wasm_bindgen(js_name = "LightVMTools")]
pub struct WasmLightVMTools {
  pub nightly: bool,
  pub backtrace: bool,
  pub explain: bool,
  pub hint: bool,
  time_budget: TimeBudget,
  pub can_observe: bool,
  pub can_control: bool,
  pub can_debug: bool,
  pub can_unsafe: bool,
}
impl WasmLightVMTools {
  fn optimizer_vm(&self) -> LightVM {
    use crate::types::security_config::SecurityConfig;
    use std::collections::HashSet;
    let mut vm_instance = LightVM::new_node(
      SecurityConfig {
        time_budget: self.time_budget,
        ..Default::default()
      },
      self.nightly,
      self.backtrace,
      self.explain,
      self.hint,
    );
    vm_instance.caps = {
      let mut caps = HashSet::new();
      if self.can_observe {
        caps.insert(Capability::Observe);
      }
      if self.can_control {
        caps.insert(Capability::Control);
      }
      if self.can_debug {
        caps.insert(Capability::Debug);
      }
      if self.can_unsafe {
        caps.insert(Capability::Unsafe);
      }
      caps
    };
    vm_instance
  }
}
#[wasm_bindgen(js_class = "LightVMTools")]
impl WasmLightVMTools {
  #[wasm_bindgen(js_name = "optimizeBytecode")]
  pub fn optimize_bytecode(&self, bytecode: JsValue) -> Result<JsValue, JsValue> {
    use crate::modules::vmerror::VMError;
    let input_json: serde_json::Value = serde_wasm_bindgen::from_value(bytecode).map_err(|e| {
      wasm_bindgen::JsValue::from(js_sys::Error::new(&format!(
        "Invalid input structure: {}",
        e
      )))
    })?;
    let mut vm_instance = self.optimizer_vm();
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
    use crate::modules::vmerror::VMError;
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
  pub fn parse_ltc_array(&self, code: String) -> Result<String, JsValue> {
    LightVM::parse_ltc_array_internal(code)
      .map_err(|e| JsValue::from(js_sys::Error::new(&e.to_string())))
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
  use crate::types::security_config::SecurityConfig;
  fn vm_with_control_capability() -> WasmLightVM {
    let mut inner = LightVM::new_node(SecurityConfig::default(), false, false, false, true);
    inner.caps.insert(Capability::Control);
    WasmLightVM { inner }
  }
  #[test]
  fn test_config_parsing() {
    let json_data = serde_json::json!({
        "caps": [0, 2],
        "runtimeConfig": { "nightly": true },
        "errorOptions": { "hint": true }
    });
    let config: VmWasmConfig = serde_json::from_value(json_data).unwrap();
    assert_eq!(config.caps, vec![0, 2]);
    #[cfg(target_arch = "wasm32")]
    {
      let mut vm = WasmLightVM::new(serde_wasm_bindgen::to_value(&config).unwrap()).unwrap();
      vm.with_nightly(true);
      assert_eq!(vm.inner.nightly, true);
      vm.with_hint(false);
      assert_eq!(vm.inner.hint, false);
    }
    assert_eq!(config.runtime_config.unwrap().nightly, Some(true));
  }
  #[test]
  fn tools_optimizer_uses_normal_time_budget_for_more_optimization() {
    let bytecode = serde_json::Value::Array(
      (0..500_000)
        .map(|_| serde_json::json!(["push", 0]))
        .collect(),
    );
    let mut cheap_vm = vm_with_control_capability();
    cheap_vm
      .set_time_budget(0)
      .expect("expected a valid cheap budget");
    let cheaply_optimized = cheap_vm
      .tools()
      .optimizer_vm()
      .optimize_bytecode_internal(bytecode.clone())
      .expect("expected cheap optimization to succeed");
    let mut normal_vm = vm_with_control_capability();
    normal_vm
      .set_time_budget(1)
      .expect("expected a valid normal budget");
    let normally_optimized = normal_vm
      .tools()
      .optimizer_vm()
      .optimize_bytecode_internal(bytecode)
      .expect("expected normal optimization to succeed");
    let cheap_len = serde_json::from_str::<Vec<serde_json::Value>>(&cheaply_optimized)
      .expect("expected cheap output to be valid bytecode")
      .len();
    let normal_len = serde_json::from_str::<Vec<serde_json::Value>>(&normally_optimized)
      .expect("expected normal output to be valid bytecode")
      .len();
    assert!(
      normal_len < cheap_len,
      "expected normal optimization to remove more instructions (normal: {normal_len}, cheap: {cheap_len})"
    );
  }
}
