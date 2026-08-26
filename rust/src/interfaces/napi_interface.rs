/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(feature = "node")]
use crate::interfaces::interface::LightVM;
use crate::types::{
  capability::Capability, compile_config::CompileConfig, file_type::FileType,
  security_config::SecurityConfig, target_arch::TargetArch, time_budget::TimeBudget,
  vmconfig::VmNapiConfig,
};
use crate::utils::vmerror::VMError;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi_derive::napi;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

fn into_napi_error(vm_error: VMError) -> Error {
  Error::from_reason(vm_error.to_string())
}

fn system_error(message: impl Into<smol_str::SmolStr>) -> VMError {
  VMError::SystemError(message.into())
}

#[napi(js_name = "LightVM")]
pub struct NodeLightVM {
  inner: LightVM,
}
#[napi]
impl NodeLightVM {
  #[napi(constructor)]
  pub fn napi_new(config: VmNapiConfig) -> Result<Self> {
    let runtime_config = config.runtime_config.unwrap_or_default();
    let error_options = config.error_options.unwrap_or_default();
    let security_config = config.security_config.unwrap_or_default();
    use crate::types::value::Value;
    use crate::types::vmstate::VmState;
    use ahash::AHashMap;
    use std::collections::HashSet;
    let mut caps_set = HashSet::new();
    if config.caps_raw.is_empty() {
      caps_set.insert(Capability::Observe);
    } else {
      for cap_num in config.caps_raw {
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
            return Err(into_napi_error(system_error(format!(
              "Unknown capability: {}",
              cap_num
            ))));
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
        last_run_options: None,
        max_io: security_config.max_io.unwrap_or(100) as usize,
        max_import: security_config.max_import.unwrap_or(3) as usize,
        max_alloc: security_config.max_alloc.unwrap_or(50) as usize,
        max_call: security_config.max_call.unwrap_or(200) as usize,
        max_jump: security_config.max_jump.unwrap_or(100) as usize,
        max_ticks: security_config.max_ticks.unwrap_or(1_000_000.0) as u64,
        max_stack_size: security_config.max_stack_size.unwrap_or(128) as usize,
        allowed_imports: security_config
          .allowed_imports
          .unwrap_or_else(|| vec!["math".into(), "time".into(), "utils".into()]),
        unsafe_mode: security_config.unsafe_mode.unwrap_or(false),
        time_budget: TimeBudget::Cheap,
        nightly: runtime_config.nightly.unwrap_or(false),
        backtrace: error_options.backtrace.unwrap_or(false),
        explain: error_options.explain.unwrap_or(false),
        hint: error_options.hint.unwrap_or(true),
      },
    })
  }
  #[napi(js_name = "setMaxIo")]
  pub fn set_max_io(&mut self, value: u32) -> Result<()> {
    self.inner.max_io = value as usize;
    Ok(())
  }
  #[napi(js_name = "setMaxImport")]
  pub fn set_max_import(&mut self, value: u32) -> Result<()> {
    self.inner.max_import = value as usize;
    Ok(())
  }
  #[napi(js_name = "setMaxAlloc")]
  pub fn set_max_alloc(&mut self, value: u32) -> Result<()> {
    self.inner.max_alloc = value as usize;
    Ok(())
  }
  #[napi(js_name = "setMaxCall")]
  pub fn set_max_call(&mut self, value: u32) -> Result<()> {
    self.inner.max_call = value as usize;
    Ok(())
  }
  #[napi(js_name = "setMaxJump")]
  pub fn set_max_jump(&mut self, value: u32) -> Result<()> {
    self.inner.max_jump = value as usize;
    Ok(())
  }
  #[napi(js_name = "setMaxTicks")]
  pub fn set_max_ticks(&mut self, value: u32) -> Result<()> {
    self.inner.max_ticks = value as u64;
    Ok(())
  }
  #[napi(js_name = "setMaxStackSize")]
  pub fn set_max_stack_size(&mut self, value: u32) -> Result<()> {
    self.inner.max_stack_size = value as usize;
    Ok(())
  }
  #[napi(js_name = "setAllowedImports")]
  pub fn set_allowed_imports(&mut self, value: Vec<String>) -> Result<()> {
    self.inner.allowed_imports = value;
    Ok(())
  }
  #[napi(js_name = "setTimeBudget")]
  pub fn set_time_budget(&mut self, value: u32) -> Result<()> {
    let (budget, ticks) = match value {
      0 => (TimeBudget::Cheap, 200),
      1 => (TimeBudget::Normal, 1000),
      2 => (TimeBudget::Expensive, 5000),
      _ => {
        return Err(into_napi_error(system_error(format!(
          "Unknown time budget: {}", value
        ))));
      }
    };
    self.inner.time_budget = budget;
    self.inner.max_ticks = ticks;
    Ok(())
  }
  #[napi(js_name = "withUnsafeMode")]
  pub fn with_unsafe_mode(&mut self, enabled: bool) -> Result<()> {
    self.inner.unsafe_mode = enabled;
    Ok(())
  }
  #[napi(js_name = "withNightly")]
  pub fn with_nightly(&mut self, enabled: bool) -> Result<()> {
    self.inner.nightly = enabled;
    Ok(())
  }
  #[napi(js_name = "withBacktrace")]
  pub fn with_backtrace(&mut self, enabled: bool) -> Result<()> {
    self.inner.backtrace = enabled;
    Ok(())
  }
  #[napi(js_name = "withExplain")]
  pub fn with_explain(&mut self, enabled: bool) -> Result<()> {
    self.inner.explain = enabled;
    Ok(())
  }
  #[napi(js_name = "withHint")]
  pub fn with_hint(&mut self, enabled: bool) -> Result<()> {
    self.inner.hint = enabled;
    Ok(())
  }
  #[napi]
  pub fn load(&mut self, source: String) -> Result<()> {
    self
      .inner
      .load_internal(source)
      .map_err(into_napi_error)
  }
  #[napi]
  pub fn run(&mut self) -> Result<serde_json::Value> {
    let raw_json = self
      .inner
      .run_internal(None)
      .map_err(into_napi_error)?;
    serde_json::from_str(&raw_json).map_err(|e| {
      into_napi_error(system_error(format!("Failed to parse VM result: {}", e)))
    })
  }
  #[napi]
  pub fn compile(
    &mut self,
    target_arch: u32,
    file_type: u32,
    path: String,
  ) -> Result<serde_json::Value> {
    let arch = match target_arch {
      0 => TargetArch::AArch64,
      _ => {
        return Err(into_napi_error(system_error(format!(
          "Unknown target architecture: {}",
          target_arch
        ))));
      }
    };
    let ftype = match file_type {
      0 => FileType::Assembly,
      1 => FileType::Binary,
      _ => {
        return Err(into_napi_error(system_error(format!(
          "Unknown file type: {}",
          file_type
        ))));
      }
    };
    let config = CompileConfig {
      target_arch: arch,
      file_type: ftype,
      path: &path,
    };
    self
      .inner
      .compile_internal(config)
      .map_err(into_napi_error)?;
    let output_path = if matches!(ftype, FileType::Assembly) {
      if path.ends_with(".s") {
        path
      } else {
        format!("{}.s", path)
      }
    } else {
      path
    };
    Ok(serde_json::json!({
      "status": "success",
      "path": output_path
    }))
  }
  #[napi]
  pub fn provide(&mut self, name: String, value: serde_json::Value) -> Result<()> {
    self
      .inner
      .provide_internal(name.into(), value)
      .map_err(into_napi_error)
  }
  #[napi]
  pub fn inspect(&self) -> Result<serde_json::Value> {
    let json_str = self
      .inner
      .inspect_internal()
      .map_err(into_napi_error)?;
    serde_json::from_str(&json_str).map_err(|e| {
      into_napi_error(system_error(format!(
        "Failed to parse inspect object: {}",
        e
      )))
    })
  }
  #[napi]
  pub fn halt(&mut self) -> Result<()> {
    self
      .inner
      .halt_internal()
      .map_err(into_napi_error)
  }
  #[napi]
  pub fn on(&mut self, event_type: u32, callback: Function<String, ()>) -> Result<()> {
    use crate::types::vmevent::VmEvent;
    let event = match event_type {
      0 => VmEvent::Tick,
      1 => VmEvent::Halt,
      2 => VmEvent::Panic,
      _ => {
        return Err(into_napi_error(system_error(format!(
          "Unknown event: {}",
          event_type
        ))));
      }
    };
    let mut threadsafe_callback = callback.build_threadsafe_function().build()?;
    #[allow(deprecated)]
    {
      let env = napi::bindgen_prelude::Env::from_raw(std::ptr::null_mut());
      threadsafe_callback.unref(&env)?;
    }
    self
      .inner
      .on_internal(event, move |payload| {
        let _ = threadsafe_callback.call(payload, ThreadsafeFunctionCallMode::NonBlocking);
      })
      .map_err(|e| into_napi_error(system_error(e)))
  }
  #[napi]
  pub fn embedded(&mut self) -> Result<serde_json::Value> {
    self
      .inner
      .clear_outputs_internal()
      .map_err(into_napi_error)?;
    let _ = self
      .inner
      .run_internal(None)
      .map_err(into_napi_error)?;
    let outputs = self
      .inner
      .get_outputs_internal()
      .map_err(into_napi_error)?;
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
    let target_name = name.clone();
    let is_function = self.inner.functions.contains_key(target_name.as_str());
    if is_function {
      let raw_result = self
        .inner
        .call_exported_internal(target_name, args)
        .map_err(into_napi_error)?;
      let parsed: serde_json::Value = serde_json::from_str(&raw_result).map_err(|e| {
        into_napi_error(system_error(format!("Failed to parse export return: {}", e)))
      })?;
      if parsed["status"] == "success" {
        let result_payload = parsed
          .get("result")
          .cloned()
          .unwrap_or(serde_json::Value::Null);
        if let Some(obj) = result_payload.as_object() {
          let defined = obj.get("defined").and_then(|v| v.as_bool()).unwrap_or(true);
          if defined {
            Ok(obj.get("value").cloned().unwrap_or(serde_json::Value::Null))
          } else {
            Ok(serde_json::Value::Null)
          }
        } else {
          Ok(result_payload)
        }
      } else {
        Err(into_napi_error(system_error(
          parsed["message"]
            .as_str()
            .unwrap_or("Unknown Error")
            .to_string(),
        )))
      }
    } else {
      let raw_result = self
        .inner
        .var_exported_internal(target_name)
        .map_err(into_napi_error)?;
      let parsed: serde_json::Value = serde_json::from_str(&raw_result).map_err(|e| {
        into_napi_error(system_error(format!("Failed to parse variable: {}", e)))
      })?;
      if let Some(obj) = parsed.as_object() {
        let defined = obj.get("defined").and_then(|v| v.as_bool()).unwrap_or(true);
        if defined {
          Ok(obj.get("value").cloned().unwrap_or(serde_json::Value::Null))
        } else {
          Ok(serde_json::Value::Null)
        }
      } else {
        Ok(parsed)
      }
    }
  }
  #[napi(js_name = "blackBox")]
  pub fn napi_black_box(value: serde_json::Value) -> Result<serde_json::Value> {
    Ok(LightVM::black_box(value))
  }
  #[napi(js_name = "bench")]
  pub fn napi_bench(
    &self,
    name: String,
    setup: Function<(), serde_json::Value>,
    f: Function<serde_json::Value, serde_json::Value>,
    bytes: Option<u32>,
    samples: Option<u32>,
    target_time: Option<u32>,
  ) -> Result<()> {
    if let Some(s) = samples {
      if s == 0 {
        return Err(into_napi_error(system_error(
          "Benchmark samples must be greater than zero",
        )));
      }
    }
    if let Some(t) = target_time {
      if t == 0 {
        return Err(into_napi_error(system_error(
          "Benchmark target_time must be greater than zero",
        )));
      }
    }
    let mut bench_obj = self
      .inner
      .bench(&name)
      .map_err(into_napi_error)?;
    if let Some(b) = bytes {
      bench_obj = bench_obj.bytes(b as usize);
    }
    if let Some(s) = samples {
      bench_obj = bench_obj.samples(s as usize);
    }
    if let Some(t) = target_time {
      bench_obj = bench_obj.target_time(std::time::Duration::from_millis(t as u64));
    }
    let mut setup_error: Option<napi::Error> = None;
    let mut f_error: Option<napi::Error> = None;
    bench_obj.run(
      || match setup.call(()) {
        Ok(val) => val,
        Err(e) => {
          if setup_error.is_none() {
            setup_error = Some(e);
          }
          serde_json::Value::Null
        }
      },
      |state| match f.call(state.clone()) {
        Ok(val) => val,
        Err(e) => {
          if f_error.is_none() {
            f_error = Some(e);
          }
          serde_json::Value::Null
        }
      },
    );
    if let Some(e) = setup_error {
      return Err(into_napi_error(system_error(e.to_string())));
    }
    if let Some(e) = f_error {
      return Err(into_napi_error(system_error(e.to_string())));
    }
    Ok(())
  }
  #[napi(js_name = "optimizeBytecode")]
  pub fn napi_optimize_bytecode(
    &self,
    bytecode: serde_json::Value,
    max_io: Option<u32>,
    max_import: Option<u32>,
    max_alloc: Option<u32>,
    max_call: Option<u32>,
    max_jump: Option<u32>,
    max_ticks: Option<u32>,
    max_stack_size: Option<u32>,
    allowed_imports: Option<Vec<String>>,
    unsafe_mode: Option<bool>,
    nightly: Option<bool>,
    backtrace: Option<bool>,
    explain: Option<bool>,
    hint: Option<bool>,
  ) -> Result<serde_json::Value> {
    let input_string = serde_json::to_string(&bytecode).map_err(|e| {
      into_napi_error(system_error(format!(
        "Failed to serialize input: {}",
        e
      )))
    })?;
    let input_json: serde_json::Value = serde_json::from_str(&input_string).map_err(|e| {
      into_napi_error(system_error(format!(
        "Invalid input structure: {}",
        e
      )))
    })?;
    let is_max_io = max_io.unwrap_or(100) as usize;
    let is_max_import = max_import.unwrap_or(3) as usize;
    let is_max_alloc = max_alloc.unwrap_or(50) as usize;
    let is_max_call = max_call.unwrap_or(200) as usize;
    let is_max_jump = max_jump.unwrap_or(100) as usize;
    let is_max_ticks = max_ticks.unwrap_or(1_000_000) as u64;
    let is_max_stack_size = max_stack_size.unwrap_or(128) as usize;
    let is_allowed_imports =
      allowed_imports.unwrap_or_else(|| vec!["math".into(), "time".into(), "utils".into()]);
    let is_unsafe_mode = unsafe_mode.unwrap_or(false);
    let is_nightly = nightly.unwrap_or(false);
    let is_backtrace = backtrace.unwrap_or(false);
    let is_explain = explain.unwrap_or(false);
    let is_hint = hint.unwrap_or(true);
    let mut vm_instance = LightVM::new_node(
      SecurityConfig {
        max_io: is_max_io,
        max_import: is_max_import,
        max_alloc: is_max_alloc,
        max_call: is_max_call,
        max_jump: is_max_jump,
        max_ticks: is_max_ticks,
        max_stack_size: is_max_stack_size,
        allowed_imports: is_allowed_imports,
        unsafe_mode: is_unsafe_mode,
        time_budget: TimeBudget::Cheap,
      },
      is_nightly,
      is_backtrace,
      is_explain,
      is_hint,
    );
    vm_instance.caps = self
      .inner
      .caps
      .iter()
      .filter_map(|cap| match cap {
        Capability::Observe => Some(Capability::Observe),
        Capability::Control => Some(Capability::Control),
        Capability::Debug => Some(Capability::Debug),
        Capability::Unsafe => Some(Capability::Unsafe),
      })
      .collect();
    let opt_str = vm_instance
      .optimize_bytecode_internal(input_json)
      .map_err(into_napi_error)?;
    serde_json::from_str::<serde_json::Value>(&opt_str).map_err(|e| {
      into_napi_error(system_error(format!(
        "Internal JSON Parsing Failed: {}",
        e
      )))
    })
  }
  #[napi(js_name = "stringifyLtc")]
  pub fn napi_stringify_ltc(json: serde_json::Value) -> Result<String> {
    LightVM::stringify_ltc_internal(json).map_err(into_napi_error)
  }
  #[napi(js_name = "parseLtc")]
  pub fn napi_parse_ltc(code: String) -> Result<String> {
    LightVM::parse_ltc_internal(code).map_err(into_napi_error)
  }
  #[napi(js_name = "parseLtcArray")]
  pub fn napi_parse_ltc_array(code: String) -> Result<String> {
    LightVM::parse_ltc_array_internal(code).map_err(into_napi_error)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unknown_capability_uses_vm_error_display() {
    let config = VmNapiConfig {
      caps_raw: vec![99],
      ..Default::default()
    };
    let error = NodeLightVM::napi_new(config).err().expect("expected an error");
    assert_eq!(
      error.reason,
      system_error("Unknown capability: 99").to_string()
    );
  }

  #[test]
  fn invalid_time_budget_uses_vm_error_display() {
    let mut vm = NodeLightVM::napi_new(VmNapiConfig::default()).expect("expected a VM");
    let error = vm.set_time_budget(99).expect_err("expected an error");
    assert_eq!(
      error.reason,
      system_error("Unknown time budget: 99").to_string()
    );
  }

  #[test]
  fn invalid_compile_options_use_vm_error_display() {
    let mut vm = NodeLightVM::napi_new(VmNapiConfig::default()).expect("expected a VM");
    let arch_error = vm
      .compile(99, 0, "output".to_string())
      .expect_err("expected an architecture error");
    assert_eq!(
      arch_error.reason,
      system_error("Unknown target architecture: 99").to_string()
    );

    let file_type_error = vm
      .compile(0, 99, "output".to_string())
      .expect_err("expected a file type error");
    assert_eq!(
      file_type_error.reason,
      system_error("Unknown file type: 99").to_string()
    );
  }

  #[test]
  fn invalid_ltc_uses_vm_error_display() {
    let code = "totally_bogus_opcode 1 2;".to_string();
    let expected = LightVM::parse_ltc_internal(code.clone())
      .expect_err("expected a VM error")
      .to_string();
    let error = NodeLightVM::napi_parse_ltc(code).expect_err("expected an N-API error");
    assert_eq!(error.reason, expected);
  }

  #[test]
  fn valid_calls_keep_existing_return_values() {
    let mut vm = NodeLightVM::napi_new(VmNapiConfig::default()).expect("expected a VM");
    vm.set_time_budget(1).expect("expected a valid budget");
    assert_eq!(vm.inner.time_budget, TimeBudget::Normal);
    assert_eq!(vm.inner.max_ticks, 1000);
    assert_eq!(
      NodeLightVM::napi_parse_ltc("stop;".to_string()).expect("expected valid LTC"),
      r#"[["stop"]]"#
    );
  }
}
