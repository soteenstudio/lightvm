/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(not(feature = "node"))]
use crate::interfaces::interface::LightVM;
use crate::modules::itme::benchmark::Benchmark;
use crate::modules::versions::InfoVM;
use crate::traits::{json_value_trait::IntoJsonValue, vmevent_trait::IntoVmEvent};
#[allow(unused_imports)]
use crate::types::vmevent::VmEvent;
use crate::types::{
  capability::Capability,
  compile_config::CompileConfig,
  error_options::ErrorOptions,
  file_type::FileType,
  runtime_config::RuntimeConfig,
  security_config::SecurityConfig,
  time_budget::TimeBudget,
  value::{RunOptions, Value},
  vmconfig::VmConfig,
  vmstate::VmState,
};
use crate::utils::get_time_budget::get_time_budget;
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use half::f16;
use smol_str::SmolStr;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use unescape::unescape;
pub struct ExportedHandle {
  name: String,
  is_function: bool,
}
impl ExportedHandle {
  pub fn call(&self, vm: &mut LightVM, args: Vec<Value>) -> Value {
    if self.is_function {
      let json_args: Result<Vec<serde_json::Value>, _> =
        args.iter().map(serde_json::to_value).collect();
      let json_args = match json_args {
        Ok(values) => values,
        Err(e) => {
          eprintln!("Failed to convert arguments: {}", e);
          return Value::Undefined;
        }
      };
      let args_value = serde_json::Value::Array(json_args);
      match vm.call_exported_internal(self.name.clone(), args_value) {
        Ok(raw_result) => {
          let parsed: serde_json::Value =
            serde_json::from_str(&raw_result).unwrap_or(serde_json::Value::Null);
          if parsed["status"] == "success" {
            export_value(
              parsed
                .get("result")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            )
          } else {
            eprintln!("Error: {}", parsed["message"]);
            Value::Undefined
          }
        }
        Err(e) => {
          eprintln!("{}", e);
          Value::Undefined
        }
      }
    } else {
      match vm.var_exported_internal(self.name.clone()) {
        Ok(raw_result) => {
          let parsed: serde_json::Value =
            serde_json::from_str(&raw_result).unwrap_or(serde_json::Value::Null);
          export_value(parsed)
        }
        Err(e) => {
          eprintln!("{}", e);
          Value::Undefined
        }
      }
    }
  }
}
fn export_value(value: serde_json::Value) -> Value {
  if let Some(payload) = value.as_object() {
    if payload.get("defined").and_then(|value| value.as_bool()) == Some(false) {
      return Value::Undefined;
    }
    let value = payload
      .get("value")
      .cloned()
      .unwrap_or(serde_json::Value::Null);
    if value.is_null() {
      return Value::Undefined;
    }
    return Value::from(value);
  }
  if value.is_null() || value == serde_json::Value::String("Undefined".to_string()) {
    Value::Undefined
  } else {
    Value::from(value)
  }
}
impl From<i16> for Value {
  fn from(v: i16) -> Self {
    Value::Int16(v)
  }
}
impl From<i32> for Value {
  fn from(v: i32) -> Self {
    Value::Int32(v)
  }
}
impl From<i64> for Value {
  fn from(v: i64) -> Self {
    Value::Int64(v)
  }
}
impl From<i128> for Value {
  fn from(v: i128) -> Self {
    Value::Int128(v)
  }
}
impl From<f32> for Value {
  fn from(v: f32) -> Self {
    Value::Float32(v)
  }
}
impl From<f64> for Value {
  fn from(v: f64) -> Self {
    Value::Float64(v)
  }
}
impl From<f16> for Value {
  fn from(v: f16) -> Self {
    Value::Float16(v)
  }
}
impl From<&str> for Value {
  fn from(v: &str) -> Self {
    Value::String(SmolStr::new(v))
  }
}
impl From<String> for Value {
  fn from(v: String) -> Self {
    Value::String(SmolStr::new(v))
  }
}
impl From<SmolStr> for Value {
  fn from(v: SmolStr) -> Self {
    Value::String(v)
  }
}
impl From<bool> for Value {
  fn from(v: bool) -> Self {
    Value::Bool(v)
  }
}
impl From<Vec<Value>> for Value {
  fn from(v: Vec<Value>) -> Self {
    Value::Array(Arc::new(v))
  }
}
impl From<AHashMap<SmolStr, Value>> for Value {
  fn from(v: AHashMap<SmolStr, Value>) -> Self {
    Value::Object(Arc::new(v))
  }
}
impl From<()> for Value {
  fn from(_: ()) -> Self {
    Value::Null
  }
}
#[cfg(not(feature = "node"))]
impl LightVM {
  pub fn new<C: Into<VmConfig>>(config: C) -> Self {
    let config: VmConfig = config.into();
    let runtime_config: RuntimeConfig = config.runtime_config.unwrap_or_default();
    let error_options: ErrorOptions = config.error_options.unwrap_or_default();
    let security_config: SecurityConfig = config.security_config.unwrap_or_default();
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
      last_run_options: None,
      max_io: security_config.max_io,
      max_import: security_config.max_import,
      max_alloc: security_config.max_alloc,
      max_call: security_config.max_call,
      max_jump: security_config.max_jump,
      max_ticks: security_config.max_ticks,
      max_stack_size: security_config.max_stack_size,
      allowed_imports: security_config.allowed_imports,
      unsafe_mode: security_config.unsafe_mode,
      time_budget: security_config.time_budget,
      nightly: runtime_config.nightly,
      backtrace: error_options.backtrace,
      explain: error_options.explain,
      hint: error_options.hint,
    }
  }
  pub fn set_max_io(mut self, value: usize) -> Self {
    self.max_io = value;
    self
  }
  pub fn set_max_import(mut self, value: usize) -> Self {
    self.max_import = value;
    self
  }
  pub fn set_max_alloc(mut self, value: usize) -> Self {
    self.max_alloc = value;
    self
  }
  pub fn set_max_call(mut self, value: usize) -> Self {
    self.max_call = value;
    self
  }
  pub fn set_max_jump(mut self, value: usize) -> Self {
    self.max_jump = value;
    self
  }
  pub fn set_max_ticks(mut self, value: u64) -> Self {
    self.max_ticks = value;
    self
  }
  pub fn set_max_stack_size(mut self, value: usize) -> Self {
    self.max_stack_size = value;
    self
  }
  pub fn set_allowed_imports(mut self, value: Vec<String>) -> Self {
    self.allowed_imports = value;
    self
  }
  pub fn set_time_budget(mut self, value: TimeBudget) -> Self {
    self.max_ticks = get_time_budget(value.clone());
    self.time_budget = value;
    self
  }
  pub fn with_unsafe_mode(mut self, enabled: bool) -> Self {
    self.unsafe_mode = enabled;
    self
  }
  pub fn with_nightly(mut self, enabled: bool) -> Self {
    self.nightly = enabled;
    self
  }
  pub fn with_backtrace(mut self, enabled: bool) -> Self {
    self.backtrace = enabled;
    self
  }
  pub fn with_explain(mut self, enabled: bool) -> Self {
    self.explain = enabled;
    self
  }
  pub fn with_hint(mut self, enabled: bool) -> Self {
    self.hint = enabled;
    self
  }
  pub fn info(&mut self) -> InfoVM {
    self.info_internal()
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
  pub fn run(&mut self, options: Option<RunOptions>) -> String {
    self
      .run_internal(options)
      .unwrap_or_else(|e| format!(r#"{{"status": "error", "message": "{}"}}"#, e))
  }
  pub fn compile(&mut self, config: CompileConfig) -> String {
    let output_path = if matches!(config.file_type, FileType::Assembly) {
      if config.path.ends_with(".s") {
        config.path.to_string()
      } else {
        format!("{}.s", config.path)
      }
    } else {
      config.path.to_string()
    };
    match self.compile_internal(config) {
      Ok(_) => output_path,
      Err(e) => {
        eprintln!("{}", e);
        std::process::exit(1);
      }
    }
  }
  /// Function to export functions in the VM out.
  ///
  /// # Examples
  /// ```rust,ignore
  /// let add = vm.export("add".to_string());
  /// let args = vec![5.into(), 6.into()];
  /// let result = add.call(&mut vm, args);
  /// println!("Result from VM: {}", result);
  /// ```
  pub fn export(&self, name: String) -> ExportedHandle {
    ExportedHandle {
      is_function: self.functions.contains_key(name.as_str()),
      name,
    }
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
  pub fn on<E, F>(&mut self, event: E, callback: F) -> &mut Self
  where
    E: IntoVmEvent,
    F: Fn(String) + Send + Sync + 'static,
  {
    let vm_event = event.to_vm_event();
    let _ = self.on_internal(vm_event, callback);
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
      backtrace: self.backtrace,
      explain: self.explain,
      hint: self.hint,
      can_control: self.caps.contains(&Capability::Control),
      can_debug: self.caps.contains(&Capability::Debug),
    }
  }
}
pub struct LightVMTools {
  pub nightly: bool,
  pub backtrace: bool,
  pub explain: bool,
  pub hint: bool,
  pub can_control: bool,
  pub can_debug: bool,
}
#[cfg(not(feature = "node"))]
impl LightVMTools {
  pub fn black_box<T>(&self, value: T) -> T {
    LightVM::black_box(value)
  }
  pub fn bench(&self, name: &str) -> Result<Benchmark, VMError> {
    if !self.can_debug {
      return Err(VMError::SystemError("Capability Debug not granted".into()));
    }
    Ok(Benchmark::new(name))
  }
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
      caps: {
        let mut caps = Vec::new();
        if self.can_control {
          caps.push(Capability::Control);
        }
        if self.can_debug {
          caps.push(Capability::Debug);
        }
        caps
      },
      runtime_config: Some(RuntimeConfig {
        nightly: self.nightly,
      }),
      error_options: Some(ErrorOptions {
        backtrace: self.backtrace,
        explain: self.explain,
        hint: self.hint,
      }),
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
  pub fn stringify_ltc<T: IntoJsonValue>(&self, input: T) -> String {
    let json = match input.into_json_value() {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Failed to parse/convert input: {}", e);
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
  pub fn parse_ltc(&self, code: &str) -> String {
    match LightVM::parse_ltc_internal(code.to_string()) {
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
  pub fn parse_ltc_array(&self, code: &str) -> String {
    match LightVM::parse_ltc_array_internal(code.to_string()) {
      Ok(text) => text,
      Err(e) => {
        eprintln!("{}", e);
        std::process::exit(1);
      }
    }
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
    vm.on(VmEvent::Tick, |_| {});
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
    vm.on(VmEvent::Tick, move |_| {
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
    vm.on(VmEvent::Tick, move |data| {
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
  fn tools_capture_source_capability_state() {
    let config = VmConfig {
      caps: vec![Capability::Control],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let tools = vm.tools();
    assert!(tools.can_control);
    assert!(!tools.can_debug);
  }
  #[test]
  fn tools_bench_requires_debug_capability() {
    let config = VmConfig {
      caps: vec![Capability::Observe],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let tools = vm.tools();
    assert!(tools.bench("bench").is_err());
  }
  #[test]
  fn tools_bench_succeeds_with_debug_capability() {
    let config = VmConfig {
      caps: vec![Capability::Debug],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let tools = vm.tools();
    assert!(tools.bench("bench").is_ok());
  }
  #[test]
  fn tools_optimizer_vm_preserves_denied_control_decision() {
    let config = VmConfig {
      caps: vec![Capability::Observe],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let tools = vm.tools();
    let mut optimizer_vm = LightVM::new(VmConfig {
      caps: if tools.can_control {
        vec![Capability::Control]
      } else {
        vec![Capability::Observe]
      },
      runtime_config: Some(RuntimeConfig {
        nightly: tools.nightly,
      }),
      error_options: Some(ErrorOptions {
        backtrace: tools.backtrace,
        explain: tools.explain,
        hint: tools.hint,
      }),
      ..Default::default()
    });
    let result = optimizer_vm.optimize_bytecode_internal(json!([["noop"]]));
    assert!(result.is_err());
  }
  #[test]
  fn tools_optimizer_vm_preserves_allowed_control_decision() {
    let config = VmConfig {
      caps: vec![Capability::Control],
      ..Default::default()
    };
    let mut vm = LightVM::new(config);
    let tools = vm.tools();
    let mut optimizer_vm = LightVM::new(VmConfig {
      caps: if tools.can_control {
        vec![Capability::Control]
      } else {
        vec![Capability::Observe]
      },
      runtime_config: Some(RuntimeConfig {
        nightly: tools.nightly,
      }),
      error_options: Some(ErrorOptions {
        backtrace: tools.backtrace,
        explain: tools.explain,
        hint: tools.hint,
      }),
      ..Default::default()
    });
    let result = optimizer_vm.optimize_bytecode_internal(json!([["stop"]]));
    assert!(result.is_ok());
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
  #[test]
  fn time_budget_cheap_is_configured() {
    let vm = LightVM::new(VmConfig::default()).set_time_budget(TimeBudget::Cheap);
    assert_eq!(vm.time_budget, TimeBudget::Cheap);
  }
  #[test]
  fn time_budget_normal_is_configured() {
    let vm = LightVM::new(VmConfig::default()).set_time_budget(TimeBudget::Normal);
    assert_eq!(vm.time_budget, TimeBudget::Normal);
  }
  #[test]
  fn time_budget_expensive_is_configured() {
    let vm = LightVM::new(VmConfig::default()).set_time_budget(TimeBudget::Expensive);
    assert_eq!(vm.time_budget, TimeBudget::Expensive);
  }
  #[test]
  fn export_creates_function_and_variable_handles_before_calls() {
    let mut vm = LightVM::new(VmConfig {
      caps: vec![Capability::Control, Capability::Observe],
      runtime_config: Some(RuntimeConfig { nightly: true }),
      ..Default::default()
    });
    vm.load(json!([
      ["jump", 7],
      ["func", "add", 2, 2, 6, "a", "b"],
      ["get", "a"],
      ["get", "b"],
      ["add", "int"],
      ["return"],
      ["stop"],
      ["export", "add"],
      ["val", "x"],
      ["push", 5],
      ["set", "x"],
      ["export", "x"],
      ["val", "unset"],
      ["export", "unset"]
    ]));
    vm.run(None);
    let add_func = vm.export("add".to_string());
    let x_var = vm.export("x".to_string());
    let unset_var = vm.export("unset".to_string());
    assert_eq!(
      add_func.call(&mut vm, vec![5.into(), 6.into()]),
      Value::Int64(11)
    );
    assert_eq!(x_var.call(&mut vm, vec![]), Value::Int64(5));
    assert_eq!(unset_var.call(&mut vm, vec![]), Value::Undefined);
  }
  #[test]
  fn export_only_requires_shared_access() {
    let vm = LightVM::new(VmConfig::default());
    let shared_vm = &vm;
    let _handle = shared_vm.export("value".to_string());
  }
}
