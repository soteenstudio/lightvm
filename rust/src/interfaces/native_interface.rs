/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(not(feature = "node"))]
use crate::interfaces::interface::LightVM;
use crate::types::{
  capability::Capability,
  value::{RunOptions, Value},
  vmconfig::VmConfig,
  vmevent::VmEvent,
  vmstate::VmState,
};
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use unescape::unescape;
pub trait IntoJsonValue {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error>;
}
impl IntoJsonValue for &str {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(self).or_else(|_| Ok(serde_json::Value::String(self.to_string())))
  }
}
impl IntoJsonValue for String {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&self).or(Ok(serde_json::Value::String(self)))
  }
}
impl IntoJsonValue for serde_json::Value {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
    Ok(self)
  }
}
#[cfg(not(feature = "node"))]
impl LightVM {
  pub fn new(config: VmConfig) -> Self {
    let mut caps_set = HashSet::new();
    if config.caps.is_empty() {
      caps_set.insert(Capability::Observe);
    } else {
      for c in config.caps {
        caps_set.insert(c);
      }
    }
    Self {
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
      nightly: config.nightly,
      explain: config.explain,
    }
  }
  /// Function used to load bytecode before execution
  pub fn load<T: IntoJsonValue>(&mut self, source: T) -> &mut Self {
    let source_value = source.into_json_value().unwrap_or_else(|err| {
      eprintln!("Failed to process load input: {}", err);
      std::process::exit(1);
    });
    let payload = if source_value.is_string() {
      source_value.as_str().unwrap_or("").to_string()
    } else {
      source_value.to_string()
    };
    if let Err(err) = self.load_internal(payload) {
      eprintln!("{}", err);
      std::process::exit(1);
    }
    self
  }
  /// Function to start bytecode execution.
  ///
  /// # Examples
  /// ```rust,ignore
  /// let raw = r#"[
  ///   ["push", 5],
  ///   ["val", "x"],
  ///   ["set", "x"]
  /// ]"#;
  /// vm.load(vm.tools().optimize_bytecode(raw).clone())
  ///   .run(None);
  /// ```
  pub fn run(&mut self, options: Option<RunOptions>) {
    let _ = self.run_internal(options);
  }
  /// Function to export functions in the VM out.
  ///
  /// # Examples
  /// ```rust,ignore
  /// let mut add = vm.export("add".to_string());
  /// let args = vec![serde_json::json!(5), serde_json::json!(6)];
  /// if let Some(result) = add_func(args) {
  ///    println!("Result from VM: {}", result);
  /// }
  /// ```
  pub fn export(
    &mut self,
    name: String,
  ) -> Box<dyn FnMut(Vec<serde_json::Value>) -> Option<serde_json::Value> + '_> {
    let function_name = name.clone();
    Box::new(move |args| {
      let args_value = serde_json::Value::Array(args);
      match self.call_exported_internal(function_name.clone(), args_value) {
        Ok(raw_result) => {
          let parsed: serde_json::Value =
            serde_json::from_str(&raw_result).unwrap_or(serde_json::Value::Null);
          if parsed.is_null() || parsed == "Undefined" {
            return None;
          }
          if parsed.is_object() {
            parsed
              .as_object()
              .and_then(|obj| obj.values().next().cloned())
          } else {
            Some(parsed)
          }
        }
        Err(e) => {
          eprintln!("{}", e);
          std::process::exit(1);
        }
      }
    })
  }
  /// Function to inject data/variables into the VM.
  ///
  /// # Examples
  /// ```rust,ignore
  /// vm.provide(serde_json::json!({
  ///   "name": "John Doe",
  ///   "force": 2021
  /// }));
  /// let raw = r#"[
  ///   ["get", "name"],
  ///   ["println"],
  ///   ["get", "force"],
  ///   ["println"]
  /// ]"#;
  /// ```
  pub fn provide(&mut self, data: serde_json::Value) -> &mut Self {
    if let serde_json::Value::Object(map) = data {
      for (name, val) in map {
        if let Err(e) = self.provide_internal(name.into(), val) {
          eprintln!("{}", e);
          std::process::exit(1);
        }
      }
    }
    self
  }
  /// Function to force/manually stop VM.
  ///
  /// # Examples
  /// ```rust,ignore
  /// vm.halt();
  /// vm.run(None); // will not be executed
  /// println!("The VM has been terminated.");
  /// ```
  pub fn halt(&mut self) {
    let _ = self.halt_internal();
  }
  pub fn on<F>(&mut self, event_type: &str, callback: F) -> &mut Self
  where
    F: Fn(String) + Send + Sync + 'static,
  {
    let event = match event_type {
      "tick" => VmEvent::Tick,
      "halt" => VmEvent::Halt,
      "panic" => VmEvent::Panic,
      _ => {
        eprintln!("Unknown event: {}", event_type);
        std::process::exit(1);
      }
    };
    let _ = self.on_internal(event, callback);
    self
  }
  /// Function to view state, number of instructions, and capability.
  ///
  /// # Examples
  /// ```rust,ignore
  /// let report = vm.inspect();
  /// println!("{}", serde_json::to_string_pretty(&report).unwrap());
  /// ```
  pub fn inspect(&self) -> serde_json::Value {
    match self.inspect_internal() {
      Ok(json_str) => serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Null),
      Err(_) => serde_json::Value::Null,
    }
  }
  pub fn embedded(&mut self) -> serde_json::Value {
    let _ = self.clear_outputs_internal();
    let _ = self.run_internal(None);
    let outputs = self.get_outputs_internal().unwrap_or_default();
    serde_json::json!({
      "value": serde_json::Value::Null,
      "outputs": outputs,
      "halted": true
    })
  }
  /// Functions used to call utilities
  pub fn tools(&mut self) -> LightVMTools {
    LightVMTools {
      nightly: self.nightly,
    }
  }
}
pub struct LightVMTools {
  pub nightly: bool,
}
#[cfg(not(feature = "node"))]
impl LightVMTools {
  /// Optimizes raw JSON bytecode and serializes it to a string
  ///
  /// # Examples
  /// ```rust,ignore
  /// let tools = vm.tools();
  /// let optimized = tools.optimize_bytecode(raw);
  /// println!("{}", optimized);
  /// ```
  pub fn optimize_bytecode<T: IntoJsonValue>(&self, input: T) -> serde_json::Value {
    let mut bytecode: serde_json::Value = input.into_json_value().unwrap_or_else(|err| {
      eprintln!("\nFailed to parse JSON input: {}", err);
      std::process::exit(1);
    });
    if bytecode.is_string() {
      let raw_str = bytecode.as_str().unwrap_or("");
      bytecode = serde_json::from_str(raw_str).unwrap_or_else(|err| {
        eprintln!("\nFailed to parse JSON string: {}", err);
        std::process::exit(1);
      });
    }
    let config = crate::types::vmconfig::VmConfig {
      nightly: self.nightly,
      ..Default::default()
    };
    let opt_str = LightVM::new(config)
      .optimize_bytecode_internal(bytecode)
      .unwrap_or_else(|err| {
        eprintln!("\n{}", err);
        std::process::exit(1);
      });
    serde_json::from_str::<serde_json::Value>(&opt_str).unwrap_or_else(|e| {
      let format_err = VMError::SystemError(format!("Internal JSON Parsing Failed: {}", e).into());
      eprintln!("\n{}", format_err);
      std::process::exit(1);
    })
  }
  /// Converts raw JSON bytecode into a readable LTC assembly string
  ///
  /// # Examples
  /// ```rust,ignore
  /// let tools = vm.tools();
  /// let stringify = tools.stringify_ltc(raw);
  /// println!("{:#}", stringify.clone());
  /// ```
  pub fn stringify_ltc(&self, json_str: &str) -> String {
    let json: serde_json::Value = match serde_json::from_str(json_str) {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Failed to parse JSON: {}", e);
        std::process::exit(1);
      }
    };
    match LightVM::stringify_ltc_internal(json) {
      Ok(text) => unescape(&text).unwrap_or(text),
      Err(e) => {
        eprintln!("{}", e);
        std::process::exit(1);
      }
    }
  }
  /// Parses LTC code and serializes the instructions to a JSON string
  ///
  /// # Examples
  /// ```rust,ignore
  /// let tools = vm.tools();
  /// let parsed = tools.parse_ltc(raw);
  /// println!("{:#}", parsed.clone());
  /// ```
  pub fn parse_ltc(&self, code: String) -> String {
    match LightVM::parse_ltc_internal(code) {
      Ok(text) => text,
      Err(e) => {
        eprintln!("{}", e);
        std::process::exit(1);
      }
    }
  }
  /// Parses an LTC string into a JSON array
  ///
  /// # Examples
  /// ```rust,ignore
  /// let tools = vm.tools();
  /// let json = tools.parse_ltc_array(raw);
  /// println!("{:#}", json.clone());
  /// ```
  pub fn parse_ltc_array(&self, code: String) -> serde_json::Value {
    LightVM::parse_ltc_array_internal(code)
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::vmconfig::VmConfig;
  use serde_json::json;
  use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
  };
  #[test]
  fn new_creates_vm() {
    let config = VmConfig {
      caps: vec![],
      ..Default::default()
    };
    let vm = LightVM::new(config);
    assert!(vm.bytecode.is_empty());
    assert_eq!(vm.state, VmState::Idle);
  }
  #[test]
  fn on_registers_listener() {
    let config = VmConfig {
      caps: vec![],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    vm.on("tick", |_| {});
    assert_eq!(vm.listeners.get(&VmEvent::Tick).unwrap().len(), 1);
  }
  #[test]
  fn tick_event_calls_listener() {
    let config = VmConfig {
      caps: vec![],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let called = Arc::new(AtomicBool::new(false));
    let flag = called.clone();
    vm.on("tick", move |_| {
      flag.store(true, Ordering::SeqCst);
    });
    vm.emit(VmEvent::Tick, json!({"state":"start"}));
    assert!(called.load(Ordering::SeqCst));
  }
  #[test]
  fn tick_event_sends_payload() {
    let config = VmConfig {
      caps: vec![],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let payload = Arc::new(Mutex::new(String::new()));
    let out = payload.clone();
    vm.on("tick", move |data| {
      *out.lock().unwrap() = data;
    });
    vm.emit(VmEvent::Tick, json!({"hello":"world"}));
    assert_eq!(*payload.lock().unwrap(), r#"{"hello":"world"}"#);
  }
  #[test]
  fn provide_adds_imports() {
    let config = VmConfig {
      caps: vec![Capability::Control],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    vm.provide(json!({
        "foo": 123,
        "bar": "hello"
    }));
    assert_eq!(vm._imports.len(), 2);
  }
  #[test]
  fn inspect_returns_json() {
    let config = VmConfig {
      caps: vec![],
      ..Default::default()
    };
    let vm = LightVM::new(config);
    let info = vm.inspect();
    assert!(info.is_object());
    assert!(info.get("state").is_some());
  }
  #[test]
  fn tools_exists() {
    let config = VmConfig {
      caps: vec![],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let _tools = vm.tools();
  }
  #[test]
  fn embedded_returns_object() {
    let config = VmConfig {
      caps: vec![Capability::Observe, Capability::Control],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let result = vm.embedded();
    assert!(result.is_object());
    assert!(result.get("outputs").is_some());
  }
}
