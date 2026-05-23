#![cfg(not(feature = "node"))]
use crate::types::{
  capability::Capability,
  value::{RunOptions, Value},
  vmevent::VmEvent,
  vmstate::VmState,
};
use std::collections::HashSet;
use ahash::AHashMap;
use crate::utils::vmerror::VMError;
use crate::interfaces::interface::LightVM;
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
  pub fn load(&mut self, source: serde_json::Value) -> &mut Self {
    let payload = if source.is_string() {
      source.as_str().unwrap_or("").to_string()
    } else {
      source.to_string()
    };
    if let Err(err) = self.load_internal(payload) {
      eprintln!("{}", err.format());
      std::process::exit(1);
    }
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
  pub fn optimize_bytecode(&self, bytecode: serde_json::Value) -> serde_json::Value {
    let opt_str = LightVM::optimize_bytecode_internal(bytecode).unwrap_or_else(|err| {
      eprintln!("\n{}", err.format());
      std::process::exit(1);
    });
    serde_json::from_str::<serde_json::Value>(&opt_str).unwrap_or_else(|e| {
      let format_err = VMError::SystemError(format!("Internal JSON Parsing Failed: {}", e).into());
      eprintln!("\n{}", format_err.format());
      std::process::exit(1);
    })
  }
  pub fn stringify_ltc(&self, json: serde_json::Value) -> Result<String, String> {
    LightVM::stringify_ltc_internal(json)
  }
  pub fn parse_ltc(&self, code: String) -> Result<String, VMError> {
    LightVM::parse_ltc_internal(code)
  }
  pub fn parse_ltc_array(&self, code: String) -> serde_json::Value {
    LightVM::parse_ltc_array_internal(code)
  }
}