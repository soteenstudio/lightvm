/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

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
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use smol_str::SmolStr;
use std::collections::HashSet;
use std::fs;
#[napi(js_name = "LightVM")]
pub struct LightVM {
  bytecode: Vec<Instructions>,
  listeners: AHashMap<VmEvent, Vec<ThreadsafeFunction<String>>>,
  caps: HashSet<Capability>,
  state: VmState,
  _outputs: Vec<String>,
  last_value: Value,
  functions: AHashMap<SmolStr, FuncMetadata>,
  exported: HashSet<SmolStr>,
  _imports: AHashMap<String, Value>,
}
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
      last_value: Value::Undefined,
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
  #[napi]
  pub fn load(&mut self, source: napi::JsUnknown) -> Result<(), napi::Error> {
    match source.get_type()? {
      napi::ValueType::String => {
        let s: String = source
          .coerce_to_string()?
          .into_utf8()?
          .as_str()?
          .to_string();
        if s.trim().starts_with('[') {
          self.bytecode = serde_json::from_str(&s)
            .map_err(|e| napi::Error::from_reason(format!("Gagal parse JSON string: {}", e)))?;
        } else {
          let path = std::path::Path::new(&s);
          if path.exists() {
            let code =
              fs::read_to_string(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
            self.bytecode = crate::utils::loader::parse_ltc(&code);
          } else {
            return Err(napi::Error::from_reason(
              "File gak ketemu dan bukan JSON valid",
            ));
          }
        }
      }
      napi::ValueType::Object => {
        let json_str = source
          .coerce_to_string()?
          .into_utf8()?
          .as_str()?
          .to_string();
        self.bytecode = serde_json::from_str(&json_str)
          .map_err(|e| napi::Error::from_reason(format!("Gagal parse bytecode. Tips: Pastikan kirim JSON.stringify(bytecode) dari JS. Error: {}", e)))?;
      }
      _ => return Err(napi::Error::from_reason("Tipe data load gak disupport")),
    }
    self.index_metadata();
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
