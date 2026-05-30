/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::optimizer::optimize_bytecode;
use crate::types::{
  capability::Capability,
  instructions::Instructions,
  value::{FuncMetadata, RunOptions, Value},
  vmevent::VmEvent,
  vmstate::VmState,
};
use crate::utils::vmerror::VMError;
use crate::vm::run::run;
use ahash::AHashMap;
use smol_str::SmolStr;
use std::collections::HashSet;
pub type VmCallback = Box<dyn Fn(String) + Send + Sync>;
pub type VmEventMap = AHashMap<VmEvent, Vec<VmCallback>>;
pub struct LightVM {
  pub bytecode: Vec<Instructions>,
  pub listeners: VmEventMap,
  pub caps: HashSet<Capability>,
  pub state: VmState,
  pub _outputs: Vec<String>,
  pub _last_value: Value,
  pub functions: AHashMap<SmolStr, FuncMetadata>,
  pub exported: HashSet<SmolStr>,
  pub _imports: AHashMap<SmolStr, Value>,
}
impl LightVM {
  #[inline(always)]
  pub fn require(&self, cap: Capability) -> Result<(), VMError> {
    if !self.caps.contains(&cap) {
      return Err(VMError::SystemError(smol_str::SmolStr::new(format!(
        "Capability {:?} not granted",
        cap
      ))));
    }
    Ok(())
  }
  pub fn index_metadata(&mut self) {
    self.functions.clear();
    self.exported.clear();
    let mut itoa_buf = itoa::Buffer::new();
    for (i, instr) in self.bytecode.iter().enumerate() {
      match instr {
        Instructions::Func(name, params, start, end, names) => {
          let idx_str = itoa_buf.format(i);
          let mut key = String::with_capacity(6 + idx_str.len());
          key.push_str("__idx_");
          key.push_str(idx_str);
          let meta = FuncMetadata {
            params_count: *params,
            param_names: names.to_vec(),
            start: *start,
            end: *end,
          };
          self.functions.insert(SmolStr::from(key), meta.clone());
          self.functions.insert(name.clone(), meta);
        }
        Instructions::Export(name) => {
          self.exported.insert(name.clone());
        }
        _ => {}
      }
    }
  }
  pub fn emit(&self, event: VmEvent, payload: serde_json::Value) {
    if let Some(list) = self.listeners.get(&event) {
      let json_payload = payload.to_string();
      for listener in list {
        listener(json_payload.clone());
      }
    }
  }
  pub fn load_internal(&mut self, source: String) -> Result<(), VMError> {
    let trimmed = source.trim();
    if trimmed.starts_with('[') {
      let raw_list: Vec<serde_json::Value> = serde_json::from_str(trimmed).map_err(|e| {
        VMError::SystemError(smol_str::SmolStr::new(format!(
          "Failed to parse JSON: {}",
          e
        )))
      })?;
      self.bytecode = raw_list.iter().map(Instructions::from_json_array).collect();
    } else {
      let path = std::path::Path::new(trimmed);
      if path.exists() {
        let code = std::fs::read_to_string(path)
          .map_err(|e| VMError::SystemError(smol_str::SmolStr::new(e.to_string())))?;
        self.bytecode = crate::utils::loader::parse_ltc(&code);
      } else {
        return Err(VMError::InvalidOpcode {
          ip: 0,
          code: smol_str::SmolStr::new("INVALID_SOURCE"),
        });
      }
    }
    self.index_metadata();
    Ok(())
  }
  pub fn run_internal(&mut self, _options: Option<RunOptions>) -> Result<(), VMError> {
    self.require(Capability::Control)?;
    if self.bytecode.is_empty() {
      return Err(VMError::InvalidOpcode {
        ip: 0,
        code: smol_str::SmolStr::new("EMPTY_BYTECODE"),
      });
    }
    self.state = VmState::Running;
    self.emit(VmEvent::Tick, serde_json::json!({ "state": "start" }));
    let bytecode_str = serde_json::to_string(&self.bytecode).map_err(|e| {
      VMError::SystemError(smol_str::SmolStr::new(format!(
        "Failed to stringify bytecode: {}",
        e
      )))
    })?;
    let options = RunOptions {
      entry: None,
      args: Vec::new(),
      capture_return: false,
      imports: self._imports.clone(),
    };
    run(&bytecode_str, Some(options));
    Ok(())
  }
  #[inline]
  pub fn on_internal<F>(&mut self, event: VmEvent, callback: F) -> Result<(), String>
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
  pub fn provide_internal(
    &mut self,
    name: SmolStr,
    value: serde_json::Value,
  ) -> Result<(), VMError> {
    self.require(Capability::Control)?;
    let val: Value = value.into();
    self._imports.insert(name, val);
    Ok(())
  }
  #[inline]
  pub fn inspect_internal(&self) -> Result<String, VMError> {
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
  #[inline]
  pub fn halt_internal(&mut self) -> Result<(), VMError> {
    self.require(Capability::Unsafe)?;
    self.state = VmState::Halted;
    self.emit(VmEvent::Halt, serde_json::Value::Null);
    Ok(())
  }
  pub fn call_exported_internal(
    &mut self,
    name: String,
    args_raw: serde_json::Value,
  ) -> Result<String, VMError> {
    self.require(Capability::Control)?;
    if !self.exported.contains(name.as_str()) {
      return Err(VMError::InvalidOpcode {
        ip: 0,
        code: SmolStr::new(format!("NOT_EXPORTED:{}", name)),
      });
    }
    let fn_meta = self
      .functions
      .get(name.as_str())
      .ok_or_else(|| VMError::InvalidOpcode {
        ip: 0,
        code: SmolStr::new(format!("NOT_FOUND:{}", name)),
      })?;
    let json_args = args_raw.to_string();
    let args: Vec<Value> = serde_json::from_str(&json_args)
      .map_err(|e| VMError::SystemError(SmolStr::new(format!("Invalid args: {}", e))))?;
    self.state = VmState::Running;
    let bytecode_str = serde_json::to_string(&self.bytecode).map_err(|e| {
      VMError::SystemError(SmolStr::new(format!("Failed to stringify bytecode: {}", e)))
    })?;
    let options = RunOptions {
      entry: Some(fn_meta.start),
      args,
      capture_return: true,
      imports: self._imports.clone(),
    };
    let hasil_run = run(&bytecode_str.clone(), Some(options));
    Ok(hasil_run)
  }
  #[inline]
  pub fn get_outputs_internal(&mut self) -> Result<Vec<String>, VMError> {
    self.require(Capability::Observe)?;
    Ok(std::mem::take(&mut self._outputs))
  }
  #[inline]
  pub fn clear_outputs_internal(&mut self) -> Result<(), VMError> {
    self.require(Capability::Control)?;
    self._outputs.clear();
    Ok(())
  }
  pub fn optimize_bytecode_internal(bytecode_raw: serde_json::Value) -> Result<String, VMError> {
    let json_str = bytecode_raw.to_string();
    let raw_list: Vec<serde_json::Value> = serde_json::from_str(&json_str)
      .map_err(|e| VMError::SystemError(format!("Invalid JSON format: {}", e).into()))?;
    let bytecode: Vec<Instructions> = raw_list.iter().map(Instructions::from_json_array).collect();
    let optimized = optimize_bytecode::optimize_bytecode(bytecode);
    serde_json::to_string(&optimized)
      .map_err(|e| VMError::SystemError(format!("Failed to stringify: {}", e).into()))
  }
  pub fn parse_ltc_internal(code: String) -> Result<String, VMError> {
    let instructions = crate::utils::loader::parse_ltc(&code);
    serde_json::to_string(&instructions).map_err(|e| {
      VMError::SystemError(SmolStr::from(format!(
        "Failed to stringify parsed LTC: {}",
        e
      )))
    })
  }
  pub fn parse_ltc_array_internal(code: String) -> serde_json::Value {
    let instructions = crate::utils::loader::parse_ltc_to_vec(&code);
    serde_json::to_value(&instructions).unwrap_or(serde_json::Value::Array(vec![]))
  }
  pub fn stringify_ltc_internal(bytecode_raw: serde_json::Value) -> Result<String, String> {
    let json_str = bytecode_raw.to_string();
    let raw_list: Vec<serde_json::Value> =
      serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON format: {}", e))?;
    let instructions: Vec<Instructions> =
      raw_list.iter().map(Instructions::from_json_array).collect();
    Ok(crate::utils::loader::stringify_ltc(instructions))
  }
}
