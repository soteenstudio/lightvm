/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

#[allow(clippy::type_complexity)]
pub mod instructions;
pub mod optimizer;
pub mod types;
pub mod utils;
pub mod vm;
use crate::types::{
  capability::Capability,
  instructions::Instructions,
  value::{FuncMetadata, RunOptions, Value},
  vmevent::VmEvent,
  vmstate::VmState,
};
use crate::vm::run::run;
use ahash::AHashMap;
#[cfg(feature = "node")]
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
#[cfg(feature = "node")]
use napi::Env;
#[cfg(feature = "node")]
use napi_derive::napi;
use smol_str::SmolStr;
use std::collections::HashSet;
#[cfg_attr(feature = "node", napi(js_name = "LightVM"))]
pub struct LightVM {
  bytecode: Vec<Instructions>,
  #[cfg(feature = "node")]
  listeners: AHashMap<VmEvent, Vec<ThreadsafeFunction<String>>>,
  #[cfg(not(feature = "node"))]
  listeners: AHashMap<VmEvent, Vec<Box<dyn Fn(String) + Send + Sync>>>,
  caps: HashSet<Capability>,
  state: VmState,
  _outputs: Vec<String>,
  _last_value: Value,
  functions: AHashMap<SmolStr, FuncMetadata>,
  exported: HashSet<SmolStr>,
  _imports: AHashMap<String, Value>,
}
#[cfg(not(feature = "node"))]
impl LightVM {
  pub fn new(caps: Vec<Capability>) -> Self {
    let mut caps_set = HashSet::new();
    if caps.is_empty() {
      caps_set.insert(Capability::Observe);
    } else {
      for c in caps {
        caps_set.insert(c);
      }
    }
    Self {
      bytecode: Vec::new(),
      listeners: AHashMap::new(),
      caps: caps_set,
      state: VmState::Idle,
      _outputs: Vec::new(),
      _last_value: Value::Undefined,
      functions: AHashMap::new(),
      exported: HashSet::new(),
      _imports: AHashMap::new(),
    }
  }
  fn require(&self, cap: Capability) -> Result<(), String> {
    if !self.caps.contains(&cap) {
      return Err(format!("Capability {:?} not granted", cap));
    }
    Ok(())
  }
  fn index_metadata(&mut self) {
    self.functions.clear();
    self.exported.clear();
    let mut itoa_buf = itoa::Buffer::new();
    for (i, instr) in self.bytecode.iter().enumerate() {
      if let Instructions::Func(_name, params, start, end, _names) = instr {
        let idx_str = itoa_buf.format(i);
        let mut key = String::with_capacity(6 + idx_str.len());
        key.push_str("__idx_");
        key.push_str(idx_str);
        self.functions.insert(
          SmolStr::from(key),
          FuncMetadata {
            params_count: *params,
            param_names: Vec::new(),
            start: *start,
            end: *end,
          },
        );
      }
    }
  }
  fn emit(&self, event: VmEvent, payload: serde_json::Value) {
    if let Some(list) = self.listeners.get(&event) {
      let json_payload = payload.to_string();
      for listener in list {
        #[cfg(feature = "node")]
        {
          let _ = listener.call(
            Ok(json_payload.clone()),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
          );
        }
        #[cfg(not(feature = "node"))]
        {
          listener(json_payload.clone());
        }
      }
    }
  }
  fn load_internal(&mut self, source: String) -> Result<(), String> {
    let trimmed = source.trim();
    if trimmed.starts_with('[') {
      let raw_list: Vec<serde_json::Value> =
        serde_json::from_str(trimmed).map_err(|e| format!("Gagal parse JSON: {}", e))?;
      self.bytecode = raw_list.iter().map(Instructions::from_json_array).collect();
    } else {
      let path = std::path::Path::new(trimmed);
      if path.exists() {
        let code = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        self.bytecode = crate::utils::loader::parse_ltc(&code);
      } else {
        return Err("Source bukan JSON dan file tidak ditemukan".into());
      }
    }
    self.index_metadata();
    Ok(())
  }
  fn run_internal(&mut self, _options: Option<RunOptions>) -> Result<(), String> {
    self.require(Capability::Control)?;
    if self.bytecode.is_empty() {
      return Err("No bytecode loaded".into());
    }
    self.state = VmState::Running;
    self.emit(VmEvent::Tick, serde_json::json!({ "state": "start" }));
    let bytecode_str = serde_json::to_string(&self.bytecode)
      .map_err(|e| format!("Gagal stringify bytecode: {}", e))?;
    run(bytecode_str);
    Ok(())
  }
  fn on_internal<F>(&mut self, event: VmEvent, callback: F) -> Result<(), String>
  where
    F: Fn(String) + Send + Sync + 'static,
  {
    self
      .listeners
      .entry(event)
      .or_default()
      .push(Box::new(callback));
    Ok(())
  }
  fn provide_internal(&mut self, name: String, value: serde_json::Value) -> Result<(), String> {
    self.require(Capability::Control)?;
    let json_str = value.to_string();
    let val: Value =
      serde_json::from_str(&json_str).map_err(|e| format!("Invalid value format: {}", e))?;
    self._imports.insert(name, val);
    Ok(())
  }
  fn inspect_internal(&self) -> Result<String, String> {
    self.require(Capability::Observe)?;
    let info = serde_json::json!({
        "state": format!("{:?}", self.state),
        "instructions": self.bytecode.len(),
        "capabilities": self.caps.iter().collect::<Vec<_>>(),
        "functions": self.functions.len(),
        "exported": self.exported.iter().collect::<Vec<_>>()
    });
    Ok(info.to_string())
  }
  fn halt_internal(&mut self) -> Result<(), String> {
    self.require(Capability::Unsafe)?;
    self.state = VmState::Halted;
    self.emit(VmEvent::Halt, serde_json::Value::Null);
    Ok(())
  }
  fn call_exported_internal(
    &mut self,
    name: String,
    args_raw: serde_json::Value,
  ) -> Result<String, String> {
    self.require(Capability::Control)?;
    if !self.exported.contains(name.as_str()) {
      return Err(format!("Function '{}' is not exported", name));
    }
    let fn_meta = self
      .functions
      .get(name.as_str())
      .ok_or_else(|| format!("Function '{}' not found", name))?;
    let json_args = args_raw.to_string();
    let args: Vec<Value> =
      serde_json::from_str(&json_args).map_err(|e| format!("Invalid args: {}", e))?;
    self.state = VmState::Running;
    let _options = crate::types::value::RunOptions {
      entry: Some(fn_meta.start),
      args,
      capture_return: true,
      imports: ahash::AHashMap::new(),
    };
    let bytecode_str = serde_json::to_string(&self.bytecode)
      .map_err(|e| format!("Failed to stringify bytecode: {}", e))?;
    run(bytecode_str.clone());
    Ok(bytecode_str)
  }
  fn get_outputs_internal(&mut self) -> Result<Vec<String>, String> {
    self.require(Capability::Observe)?;
    Ok(std::mem::take(&mut self._outputs))
  }
  fn clear_outputs_internal(&mut self) -> Result<(), String> {
    self.require(Capability::Control)?;
    self._outputs.clear();
    Ok(())
  }
  fn optimize_bytecode_internal(bytecode_raw: serde_json::Value) -> Result<String, String> {
    let json_str = bytecode_raw.to_string();
    let raw_list: Vec<serde_json::Value> =
      serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON format: {}", e))?;
    let bytecode: Vec<Instructions> = raw_list.iter().map(Instructions::from_json_array).collect();
    let optimized = optimizer::optimize_bytecode::optimize_bytecode(bytecode);
    serde_json::to_string(&optimized).map_err(|e| format!("Gagal stringify: {}", e))
  }
  fn parse_ltc_internal(code: String) -> Result<String, String> {
    let instructions = crate::utils::loader::parse_ltc(&code);
    serde_json::to_string(&instructions)
      .map_err(|e| format!("Failed to stringify parsed LTC: {}", e))
  }
  fn parse_ltc_array_internal(code: String) -> serde_json::Value {
    let instructions = crate::utils::loader::parse_ltc_to_vec(&code);
    serde_json::to_value(&instructions).unwrap_or(serde_json::Value::Array(vec![]))
  }
  fn stringify_ltc_internal(bytecode_raw: serde_json::Value) -> Result<String, String> {
    let json_str = bytecode_raw.to_string();
    let raw_list: Vec<serde_json::Value> =
      serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON format: {}", e))?;
    let instructions: Vec<Instructions> =
      raw_list.iter().map(Instructions::from_json_array).collect();
    Ok(crate::utils::loader::stringify_ltc(instructions))
  }
  pub fn load(&mut self, source: serde_json::Value) -> &mut Self {
    let payload = if source.is_string() {
      source.as_str().unwrap_or("").to_string()
    } else {
      source.to_string()
    };
    let _ = self.load_internal(payload);
    self
  }
  pub fn run(&mut self, options: Option<RunOptions>) {
    let _ = self.run_internal(options);
  }
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
        Err(_) => None,
      }
    })
  }
  pub fn provide(&mut self, name: String, value: serde_json::Value) -> &mut Self {
    let _ = self.provide_internal(name, value);
    self
  }
  pub fn halt(&mut self) {
    let _ = self.halt_internal();
  }
  pub fn on<F>(&mut self, event: VmEvent, callback: F) -> &mut Self
  where
    F: Fn(String) + Send + Sync + 'static,
  {
    let _ = self.on_internal(event, callback);
    self
  }
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
  pub fn tools() -> LightVMTools {
    LightVMTools
  }
}
pub struct LightVMTools;
#[cfg(not(feature = "node"))]
impl LightVMTools {
  pub fn optimize_bytecode(&self, bytecode: serde_json::Value) -> Result<String, String> {
    LightVM::optimize_bytecode_internal(bytecode)
  }
  pub fn stringify_ltc(&self, json: serde_json::Value) -> Result<String, String> {
    LightVM::stringify_ltc_internal(json)
  }
  pub fn parse_ltc(&self, code: String) -> Result<String, String> {
    LightVM::parse_ltc_internal(code)
  }
  pub fn parse_ltc_array(&self, code: String) -> serde_json::Value {
    LightVM::parse_ltc_array_internal(code)
  }
}
#[cfg(feature = "node")]
#[napi]
impl LightVM {
  #[napi(constructor)]
  pub fn new(caps: Vec<Capability>) -> Self {
    let mut caps_set = HashSet::new();
    if caps.is_empty() {
      caps_set.insert(Capability::Observe);
    } else {
      for c in caps {
        caps_set.insert(c);
      }
    }
    Self {
      bytecode: Vec::new(),
      listeners: AHashMap::new(),
      caps: caps_set,
      state: VmState::Idle,
      _outputs: Vec::new(),
      _last_value: Value::Undefined,
      functions: AHashMap::new(),
      exported: HashSet::new(),
      _imports: AHashMap::new(),
    }
  }
  fn require(&self, cap: Capability) -> Result<(), napi::Error> {
    if !self.caps.contains(&cap) {
      let mut msg = String::from("Capability '");
      msg.push_str(&format!("{:?}", cap));
      msg.push_str("' not granted");
      return Err(napi::Error::from_reason(msg));
    }
    Ok(())
  }
  fn index_metadata(&mut self) {
    self.functions.clear();
    self.exported.clear();
    let mut itoa_buf = itoa::Buffer::new();
    for i in 0..self.bytecode.len() {
      if let Instructions::Func(_name, params, start, end, _names) = &self.bytecode[i] {
        let idx_str = itoa_buf.format(i);
        let mut key = String::with_capacity(6 + idx_str.len());
        key.push_str("__idx_");
        key.push_str(idx_str);
        self.functions.insert(
          SmolStr::from(key),
          FuncMetadata {
            params_count: *params,
            param_names: Vec::new(),
            start: *start,
            end: *end,
          },
        );
      }
    }
  }
  fn emit(&self, event: VmEvent, payload: serde_json::Value) {
    if let Some(list) = self.listeners.get(&event) {
      let json_payload = payload.to_string();
      for tsfn in list {
        let _ = tsfn.call(
          Ok(json_payload.to_string()),
          ThreadsafeFunctionCallMode::Blocking,
        );
      }
    }
  }
  #[napi]
  pub fn load(&mut self, env: Env, source: napi::JsUnknown) -> Result<(), napi::Error> {
    use std::fs;
    match source.get_type()? {
      napi::ValueType::String => {
        let s = source.coerce_to_string()?.into_utf8()?;
        let trimmed = s.as_str()?.trim();
        if trimmed.starts_with('[') {
          self.bytecode = serde_json::from_str(trimmed)
            .map_err(|e| napi::Error::from_reason(format!("Gagal parse JSON: {}", e)))?;
        } else {
          let path = std::path::Path::new(trimmed);
          if path.exists() {
            let code =
              fs::read_to_string(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
            self.bytecode = crate::utils::loader::parse_ltc(&code);
          }
        }
      }
      napi::ValueType::Object => {
        let json_method = env
          .get_global()?
          .get_named_property::<napi::JsObject>("JSON")?
          .get_named_property::<napi::JsFunction>("stringify")?;
        let json_str: String = json_method
          .call(None, &[source])?
          .coerce_to_string()?
          .into_utf8()?
          .as_str()?
          .to_string();
        self.bytecode = serde_json::from_str(&json_str).map_err(|e| {
          napi::Error::from_reason(format!("Gagal parse Object ke Bytecode: {}", e))
        })?;
      }
      _ => return Err(napi::Error::from_reason("Tipe data load gak disupport")),
    }
    self.index_metadata();
    Ok(())
  }
  #[napi]
  pub fn run(&mut self, options: napi::JsUnknown) -> Result<(), napi::Error> {
    self.require(Capability::Control)?;
    let _run_opts: Option<RunOptions> = match options.get_type()? {
      napi::ValueType::Null | napi::ValueType::Undefined => None,
      _ => {
        let json_str = options
          .coerce_to_string()?
          .into_utf8()?
          .as_str()?
          .to_string();
        serde_json::from_str::<RunOptions>(&json_str).ok()
      }
    };
    if self.bytecode.is_empty() {
      return Err(napi::Error::from_reason("No bytecode loaded"));
    }
    self.state = VmState::Running;
    self.emit(VmEvent::Tick, serde_json::json!({ "state": "start" }));
    let bytecode_str = serde_json::to_string(&self.bytecode).unwrap();
    run(bytecode_str);
    Ok(())
  }
  #[napi]
  pub fn on(&mut self, event: VmEvent, callback: napi::JsFunction) -> Result<(), napi::Error> {
    let tsfn: ThreadsafeFunction<String, napi::threadsafe_function::ErrorStrategy::CalleeHandled> =
      callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;
    self.listeners.entry(event).or_default().push(tsfn);
    Ok(())
  }
  #[napi]
  pub fn provide(&mut self, name: String, value: napi::JsUnknown) -> Result<(), napi::Error> {
    self.require(Capability::Control)?;
    let json_str = value.coerce_to_string()?.into_utf8()?.as_str()?.to_string();
    let val: Value =
      serde_json::from_str(&json_str).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    self._imports.insert(name, val);
    Ok(())
  }
  #[napi]
  pub fn inspect(&self) -> Result<String, napi::Error> {
    self.require(Capability::Observe)?;
    let info = serde_json::json!({
        "state": format!("{:?}", self.state),
        "instructions": self.bytecode.len(),
        "capabilities": self.caps.iter().collect::<Vec<_>>(),
        "functions": self.functions.len(),
        "exported": self.exported.iter().collect::<Vec<_>>()
    });
    Ok(info.to_string())
  }
  #[napi]
  pub fn halt(&mut self) -> Result<(), napi::Error> {
    self.require(Capability::Unsafe)?;
    self.state = VmState::Halted;
    self.emit(VmEvent::Halt, serde_json::Value::Null);
    Ok(())
  }
  #[napi]
  pub fn call_exported(
    &mut self,
    name: String,
    args_raw: napi::JsUnknown,
  ) -> Result<String, napi::Error> {
    self.require(Capability::Control)?;
    if !self.exported.contains(name.as_str()) {
      return Err(napi::Error::from_reason(format!(
        "Function '{}' is not exported",
        name
      )));
    }
    let fn_meta = self
      .functions
      .get(name.as_str())
      .ok_or_else(|| napi::Error::from_reason(format!("Function '{}' not found", name)))?;
    let json_args = args_raw
      .coerce_to_string()?
      .into_utf8()?
      .as_str()?
      .to_string();
    let args: Vec<Value> = serde_json::from_str(&json_args)
      .map_err(|e| napi::Error::from_reason(format!("Invalid args: {}", e)))?;
    self.state = VmState::Running;
    let _options = RunOptions {
      entry: Some(fn_meta.start),
      args,
      capture_return: true,
      imports: AHashMap::new(),
    };
    let bytecode_str = serde_json::to_string(&self.bytecode).unwrap();
    run(bytecode_str.clone());
    Ok(bytecode_str)
  }
  #[napi]
  pub fn get_outputs(&mut self) -> Result<Vec<String>, napi::Error> {
    self.require(Capability::Observe)?;
    Ok(std::mem::take(&mut self._outputs))
  }
  #[napi]
  pub fn clear_outputs(&mut self) -> Result<(), napi::Error> {
    self.require(Capability::Control)?;
    self._outputs.clear();
    Ok(())
  }
  #[napi]
  pub fn optimize_bytecode(bytecode_raw: napi::JsUnknown) -> Result<String, napi::Error> {
    let json_str = bytecode_raw
      .coerce_to_string()?
      .into_utf8()?
      .as_str()?
      .to_string();
    let raw_list: Vec<serde_json::Value> = serde_json::from_str(&json_str)
      .map_err(|e| napi::Error::from_reason(format!("Invalid JSON format: {}", e)))?;
    let bytecode: Vec<Instructions> = raw_list.iter().map(Instructions::from_json_array).collect();
    let optimized = optimizer::optimize_bytecode::optimize_bytecode(bytecode);
    serde_json::to_string(&optimized)
      .map_err(|e| napi::Error::from_reason(format!("Gagal stringify: {}", e)))
  }
  #[napi]
  pub fn parse_ltc(code: String) -> Result<String, napi::Error> {
    let instructions = crate::utils::loader::parse_ltc(&code);
    serde_json::to_string(&instructions)
      .map_err(|e| napi::Error::from_reason(format!("Failed to stringify parsed LTC: {}", e)))
  }
  #[napi]
  pub fn parse_ltc_array(code: String) -> serde_json::Value {
    let instructions = crate::utils::loader::parse_ltc_to_vec(&code);
    serde_json::to_value(&instructions).unwrap_or(serde_json::Value::Array(vec![]))
  }
  #[napi]
  pub fn stringify_ltc(bytecode_raw: napi::JsUnknown) -> Result<String, napi::Error> {
    let json_str = bytecode_raw
      .coerce_to_string()?
      .into_utf8()?
      .as_str()?
      .to_string();
    let raw_list: Vec<serde_json::Value> = serde_json::from_str(&json_str)
      .map_err(|e| napi::Error::from_reason(format!("Invalid JSON format: {}", e)))?;
    let instructions: Vec<Instructions> =
      raw_list.iter().map(Instructions::from_json_array).collect();
    Ok(crate::utils::loader::stringify_ltc(instructions))
  }
}
