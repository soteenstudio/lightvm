/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#![cfg(all(not(feature = "node"), not(target_arch = "wasm32")))]
#![allow(improper_ctypes_definitions)]
use crate::interfaces::interface::{LightVM, VmEventData};
use crate::modules::vmerror::VMError;
#[allow(unused_imports)]
use crate::types::vmevent::VmEvent;
use crate::types::{
  capability::Capability,
  compile_config::CompileConfig,
  error_options::ErrorOptions,
  file_type::FileType,
  runtime_config::RuntimeConfig,
  security_config::SecurityConfig,
  target_arch::TargetArch,
  time_budget::TimeBudget,
  value::{RunOptions, Value},
  vmconfig::VmConfig,
};
use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Capability ids, same values as `Capability` in the TS interface.
pub const LIGHTVM_CAPABILITY_CONTROL: c_int = 0;
pub const LIGHTVM_CAPABILITY_OBSERVE: c_int = 1;
pub const LIGHTVM_CAPABILITY_DEBUG: c_int = 2;
pub const LIGHTVM_CAPABILITY_UNSAFE: c_int = 3;
/// Event ids, same values as `VMEvent` in the TS interface.
pub const LIGHTVM_EVENT_TICK: c_int = 0;
pub const LIGHTVM_EVENT_HALT: c_int = 1;
pub const LIGHTVM_EVENT_PANIC: c_int = 2;
pub const LIGHTVM_EVENT_START: c_int = 3;
pub const LIGHTVM_EVENT_FINISH: c_int = 4;
/// Time budget ids, same values as `TimeBudget` in the TS interface.
/// `LIGHTVM_TIME_BUDGET_DEFAULT` keeps the default of the VM.
pub const LIGHTVM_TIME_BUDGET_DEFAULT: c_int = -1;
pub const LIGHTVM_TIME_BUDGET_CHEAP: c_int = 0;
pub const LIGHTVM_TIME_BUDGET_NORMAL: c_int = 1;
pub const LIGHTVM_TIME_BUDGET_EXPENSIVE: c_int = 2;
/// Target arch ids, same values as `TargetArch` in the TS interface.
pub const LIGHTVM_TARGET_ARCH_AARCH64: c_int = 0;
/// File type ids, same values as `FileType` in the TS interface.
pub const LIGHTVM_FILE_TYPE_ASSEMBLY: c_int = 0;
pub const LIGHTVM_FILE_TYPE_BINARY: c_int = 1;
/// Status codes returned by functions that do not return a pointer.
pub const LIGHTVM_OK: c_int = 0;
pub const LIGHTVM_ERR: c_int = -1;

#[repr(C)]
pub struct LightVMRuntimeConfig {
  pub nightly: bool,
}
#[repr(C)]
pub struct LightVMErrorOptions {
  pub backtrace: bool,
  pub explain: bool,
  pub hint: bool,
  pub diagnostic_links: bool,
}
#[repr(C)]
pub struct LightVMSecurityConfig {
  pub max_io: usize,
  pub max_import: usize,
  pub max_alloc: usize,
  pub max_call: usize,
  pub max_jump: usize,
  pub max_ticks: u64,
  pub max_stack_size: usize,
  pub allowed_imports: *const *const c_char,
  pub allowed_imports_len: usize,
  pub unsafe_mode: bool,
  pub time_budget: c_int,
}
#[repr(C)]
pub struct LightVMConfig {
  pub caps: *const c_int,
  pub caps_len: usize,
  pub runtime_config: LightVMRuntimeConfig,
  pub error_options: LightVMErrorOptions,
  pub security_config: LightVMSecurityConfig,
}
#[repr(C)]
pub struct LightVMRunOptions {
  pub capture_return: bool,
}
#[repr(C)]
pub struct LightVMCompileConfig {
  pub target_arch: c_int,
  pub file_type: c_int,
  pub path: *const c_char,
}
/// Handle returned by `lightvm_export`. Free it with `lightvm_export_free`.
pub struct LightVMExport {
  name: String,
  is_function: bool,
}

static DEFAULT_CAPS: [c_int; 1] = [LIGHTVM_CAPABILITY_OBSERVE];
thread_local! {
  static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}
fn set_last_error(message: String) {
  eprintln!("{}", message);
  let message = CString::new(message.replace('\0', " ")).unwrap_or_default();
  LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(message));
}
fn clear_last_error() {
  LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
}
fn fail<T>(error: VMError, fallback: T) -> T {
  set_last_error(error.to_string());
  fallback
}
fn into_c_string(text: String) -> *mut c_char {
  CString::new(text.replace('\0', " "))
    .map(CString::into_raw)
    .unwrap_or(std::ptr::null_mut())
}
fn read_c_str(ptr: *const c_char) -> Result<String, VMError> {
  if ptr.is_null() {
    return Err(VMError::SystemError("Received a null string".into()));
  }
  unsafe { CStr::from_ptr(ptr) }
    .to_str()
    .map(|text| text.to_string())
    .map_err(|error| VMError::SystemError(format!("Invalid UTF-8 input: {}", error).into()))
}
fn read_json(ptr: *const c_char) -> Result<serde_json::Value, VMError> {
  let text = read_c_str(ptr)?;
  serde_json::from_str(&text)
    .map_err(|error| VMError::SystemError(format!("Failed to parse JSON input: {}", error).into()))
}
fn read_args(ptr: *const c_char) -> Result<serde_json::Value, VMError> {
  if ptr.is_null() {
    return Ok(serde_json::Value::Array(Vec::new()));
  }
  let items = match read_json(ptr)? {
    serde_json::Value::Array(items) => items,
    _ => return Err(VMError::SystemError("Arguments must be a JSON array".into())),
  };
  let json_args: Result<Vec<serde_json::Value>, VMError> = items
    .into_iter()
    .map(Value::from)
    .map(|v| serde_json::to_value(&v).map_err(|e| VMError::SystemError(e.to_string().into())))
    .collect();
  Ok(serde_json::Value::Array(json_args?))
}
fn guard<T>(fallback: T, body: impl FnOnce() -> T) -> T {
  catch_unwind(AssertUnwindSafe(body)).unwrap_or_else(|_| {
    set_last_error("Panic caught at the C ABI boundary".to_string());
    fallback
  })
}
fn vm_mut<'a>(vm: *mut LightVM) -> Option<&'a mut LightVM> {
  if vm.is_null() {
    set_last_error("Received a null LightVM handle".to_string());
    return None;
  }
  Some(unsafe { &mut *vm })
}
/// Runs `body` on the VM and returns the same handle so calls can be chained,
/// like `return this` in the TS interface. A null handle stays null without
/// replacing the error that made it null.
fn chain(vm: *mut LightVM, body: impl FnOnce(&mut LightVM) -> Result<(), VMError>) -> *mut LightVM {
  if vm.is_null() {
    return std::ptr::null_mut();
  }
  guard(std::ptr::null_mut(), || match body(unsafe { &mut *vm }) {
    Ok(_) => vm,
    Err(error) => fail(error, std::ptr::null_mut()),
  })
}
fn to_capability(id: c_int) -> Result<Capability, VMError> {
  match id {
    LIGHTVM_CAPABILITY_CONTROL => Ok(Capability::Control),
    LIGHTVM_CAPABILITY_OBSERVE => Ok(Capability::Observe),
    LIGHTVM_CAPABILITY_DEBUG => Ok(Capability::Debug),
    LIGHTVM_CAPABILITY_UNSAFE => Ok(Capability::Unsafe),
    _ => Err(VMError::SystemError("Unknown capability".into())),
  }
}
fn to_time_budget(id: c_int) -> Result<TimeBudget, VMError> {
  match id {
    LIGHTVM_TIME_BUDGET_CHEAP => Ok(TimeBudget::Cheap),
    LIGHTVM_TIME_BUDGET_NORMAL => Ok(TimeBudget::Normal),
    LIGHTVM_TIME_BUDGET_EXPENSIVE => Ok(TimeBudget::Expensive),
    _ => Err(VMError::SystemError("Unknown time budget".into())),
  }
}
fn to_vm_event(id: c_int) -> Result<VmEvent, VMError> {
  match id {
    LIGHTVM_EVENT_TICK => Ok(VmEvent::Tick),
    LIGHTVM_EVENT_HALT => Ok(VmEvent::Halt),
    LIGHTVM_EVENT_PANIC => Ok(VmEvent::Panic),
    LIGHTVM_EVENT_START => Ok(VmEvent::Start),
    LIGHTVM_EVENT_FINISH => Ok(VmEvent::Finish),
    _ => Err(VMError::SystemError("Unknown event".into())),
  }
}
fn to_vm_config(config: &LightVMConfig) -> Result<VmConfig, VMError> {
  let mut caps = Vec::new();
  if !config.caps.is_null() && config.caps_len > 0 {
    for id in unsafe { std::slice::from_raw_parts(config.caps, config.caps_len) } {
      caps.push(to_capability(*id)?);
    }
  }
  let security = &config.security_config;
  let mut allowed_imports = Vec::new();
  if !security.allowed_imports.is_null() && security.allowed_imports_len > 0 {
    for ptr in unsafe { std::slice::from_raw_parts(security.allowed_imports, security.allowed_imports_len) } {
      allowed_imports.push(read_c_str(*ptr)?);
    }
  }
  let mut security_config = SecurityConfig {
    max_io: security.max_io,
    max_import: security.max_import,
    max_alloc: security.max_alloc,
    max_call: security.max_call,
    max_jump: security.max_jump,
    max_ticks: security.max_ticks,
    max_stack_size: security.max_stack_size,
    allowed_imports,
    unsafe_mode: security.unsafe_mode,
    ..Default::default()
  };
  if security.time_budget != LIGHTVM_TIME_BUDGET_DEFAULT {
    security_config.time_budget = to_time_budget(security.time_budget)?;
  }
  Ok(VmConfig {
    caps,
    security_config: Some(security_config),
    runtime_config: Some(RuntimeConfig {
      nightly: config.runtime_config.nightly,
    }),
    error_options: Some(ErrorOptions {
      backtrace: config.error_options.backtrace,
      explain: config.error_options.explain,
      hint: config.error_options.hint,
      diagnostic_links: config.error_options.diagnostic_links,
    }),
    ..Default::default()
  })
}
struct CallbackContext {
  callback: extern "C" fn(*const c_char, *const c_char, *mut c_void),
  user_data: *mut c_void,
}
unsafe impl Send for CallbackContext {}
unsafe impl Sync for CallbackContext {}

/// Returns the last error message of the calling thread, or null.
/// The pointer stays valid until the next failing call on this thread.
/// Do not free it.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_last_error() -> *const c_char {
  LAST_ERROR.with(|slot| {
    slot
      .borrow()
      .as_ref()
      .map(|message| message.as_ptr())
      .unwrap_or(std::ptr::null())
  })
}
/// Frees a string returned by any `lightvm_*` function.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_string_free(text: *mut c_char) {
  if !text.is_null() {
    drop(unsafe { CString::from_raw(text) });
  }
}
/// Returns the default config, same values as `DEFAULTS` in the TS interface.
/// Change the fields you need, like passing a partial config in TS.
///
/// # Examples
/// ```c
/// LightVMConfig config = lightvm_config_default();
/// config.security_config.max_ticks = 500000;
/// LightVM *vm = lightvm_new(&config);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_config_default() -> LightVMConfig {
  LightVMConfig {
    caps: DEFAULT_CAPS.as_ptr(),
    caps_len: DEFAULT_CAPS.len(),
    runtime_config: LightVMRuntimeConfig { nightly: false },
    error_options: LightVMErrorOptions {
      backtrace: false,
      explain: false,
      hint: true,
      diagnostic_links: true,
    },
    security_config: LightVMSecurityConfig {
      max_io: 100,
      max_import: 3,
      max_alloc: 50,
      max_call: 200,
      max_jump: 100,
      max_ticks: 1_000_000,
      max_stack_size: 128,
      allowed_imports: std::ptr::null(),
      allowed_imports_len: 0,
      unsafe_mode: false,
      time_budget: LIGHTVM_TIME_BUDGET_DEFAULT,
    },
  }
}
/// Creates a VM. A null `config` uses `lightvm_config_default()`.
/// Returns null on error.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_new(config: *const LightVMConfig) -> *mut LightVM {
  guard(std::ptr::null_mut(), || {
    let defaults = lightvm_config_default();
    let config = if config.is_null() {
      &defaults
    } else {
      unsafe { &*config }
    };
    match to_vm_config(config) {
      Ok(config) => Box::into_raw(Box::new(LightVM::new(config))),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Destroys a VM created by `lightvm_new`.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_free(vm: *mut LightVM) {
  if !vm.is_null() {
    drop(unsafe { Box::from_raw(vm) });
  }
}
macro_rules! c_setter {
  ($name:ident, $field:ident, $ty:ty) => {
    #[unsafe(no_mangle)]
    pub extern "C" fn $name(vm: *mut LightVM, value: $ty) -> *mut LightVM {
      chain(vm, |vm| {
        vm.$field = value;
        Ok(())
      })
    }
  };
}
c_setter!(lightvm_set_max_io, max_io, usize);
c_setter!(lightvm_set_max_import, max_import, usize);
c_setter!(lightvm_set_max_alloc, max_alloc, usize);
c_setter!(lightvm_set_max_call, max_call, usize);
c_setter!(lightvm_set_max_jump, max_jump, usize);
c_setter!(lightvm_set_max_ticks, max_ticks, u64);
c_setter!(lightvm_set_max_stack_size, max_stack_size, usize);
c_setter!(lightvm_with_unsafe_mode, unsafe_mode, bool);
c_setter!(lightvm_with_nightly, nightly, bool);
c_setter!(lightvm_with_backtrace, backtrace, bool);
c_setter!(lightvm_with_explain, explain, bool);
c_setter!(lightvm_with_hint, hint, bool);
c_setter!(lightvm_with_diagnostic_links, diagnostic_links, bool);

/// Sets the allowed imports from a JSON array of strings.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_set_allowed_imports(vm: *mut LightVM, json: *const c_char) -> *mut LightVM {
  chain(vm, |vm| {
    let list = serde_json::from_value::<Vec<String>>(read_json(json)?).map_err(|error| {
      VMError::SystemError(format!("Expected an array of strings: {}", error).into())
    })?;
    vm.allowed_imports = list;
    Ok(())
  })
}
/// Sets the time budget using one of `LIGHTVM_TIME_BUDGET_*`.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_set_time_budget(vm: *mut LightVM, budget: c_int) -> *mut LightVM {
  chain(vm, |vm| {
    vm.time_budget = to_time_budget(budget)?;
    Ok(())
  })
}
/// Loads bytecode (JSON array text) before execution.
///
/// # Examples
/// ```c
/// lightvm_load(vm, "[[\"push\", 5], [\"val\", \"x\"], [\"set\", \"x\"]]");
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_load(vm: *mut LightVM, source: *const c_char) -> *mut LightVM {
  chain(vm, |vm| {
    let payload = read_c_str(source)?;
    vm.load_internal(payload).map(|_| ())
  })
}
/// Starts bytecode execution. A null `options` runs with the defaults.
/// Returns a JSON string (free it with `lightvm_string_free`) or null on error.
///
/// # Examples
/// ```c
/// LightVMRunOptions options = { .capture_return = true };
/// char *result = lightvm_run(vm, &options);
/// lightvm_string_free(result);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_run(vm: *mut LightVM, options: *const LightVMRunOptions) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    let options = if options.is_null() {
      None
    } else {
      Some(RunOptions {
        capture_return: unsafe { &*options }.capture_return,
        ..Default::default()
      })
    };
    match vm.run_internal(options) {
      Ok(raw) => {
        clear_last_error();
        into_c_string(raw)
      }
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Injects one variable. `value_json` is the value as JSON text.
///
/// # Examples
/// ```c
/// lightvm_provide(vm, "name", "\"John Doe\"");
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_provide(vm: *mut LightVM, name: *const c_char, value_json: *const c_char) -> *mut LightVM {
  chain(vm, |vm| {
    let name = read_c_str(name)?;
    let value = read_json(value_json)?;
    vm.provide_internal(name.into(), value).map(|_| ())
  })
}
/// Injects every key of a JSON object as a variable.
///
/// # Examples
/// ```c
/// lightvm_provide_object(vm, "{\"name\": \"John Doe\", \"force\": 2021}");
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_provide_object(vm: *mut LightVM, json: *const c_char) -> *mut LightVM {
  chain(vm, |vm| {
    if let serde_json::Value::Object(map) = read_json(json)? {
      for (name, val) in map {
        vm.provide_internal(name.into(), val)?;
      }
    }
    Ok(())
  })
}
/// Forces the VM to stop.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_halt(vm: *mut LightVM) {
  guard((), || {
    if let Some(vm) = vm_mut(vm) {
      let _ = vm.halt_internal();
    }
  })
}
/// Registers an event listener using one of `LIGHTVM_EVENT_*`. The callback
/// receives the event name, the payload as JSON text (valid only during the
/// call) and `user_data`.
///
/// # Examples
/// ```c
/// lightvm_on(vm, LIGHTVM_EVENT_TICK, on_tick, NULL);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_on(
  vm: *mut LightVM,
  event: c_int,
  callback: extern "C" fn(*const c_char, *const c_char, *mut c_void),
  user_data: *mut c_void,
) -> *mut LightVM {
  chain(vm, |vm| {
    let vm_event = to_vm_event(event)?;
    let context = CallbackContext {
      callback,
      user_data,
    };
    vm.on_internal(vm_event, move |data: &VmEventData| {
      let name = CString::new(format!("{:?}", data.event)).unwrap_or_default();
      let payload = CString::new(data.payload.to_string()).unwrap_or_default();
      (context.callback)(name.as_ptr(), payload.as_ptr(), context.user_data);
    })
    .map(|_| ())
  })
}
/// Returns state, number of instructions and capabilities as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_inspect(vm: *mut LightVM) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    match vm.inspect_internal() {
      Ok(json) => into_c_string(json),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Returns the persisted paniclog records as JSON (requires Debug).
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_paniclog(vm: *mut LightVM) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    match vm.list_paniclog_internal() {
      Ok(records) => into_c_string(records),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Clears all persisted paniclog records (requires Debug).
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_clear_paniclog(vm: *mut LightVM) -> c_int {
  guard(LIGHTVM_ERR, || {
    let Some(vm) = vm_mut(vm) else {
      return LIGHTVM_ERR;
    };
    match vm.clear_paniclog_internal() {
      Ok(_) => LIGHTVM_OK,
      Err(error) => fail(error, LIGHTVM_ERR),
    }
  })
}
/// Runs the loaded bytecode and returns `{ value, outputs, halted }` as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_embedded(vm: *mut LightVM) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    into_c_string(vm.embedded().to_string())
  })
}
/// Returns the default compile config, same values as `CompileConfig::default()`.
///
/// # Examples
/// ```c
/// LightVMCompileConfig config = lightvm_compile_config_default();
/// config.file_type = LIGHTVM_FILE_TYPE_ASSEMBLY;
/// char *path = lightvm_compile(vm, &config);
/// lightvm_string_free(path);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_compile_config_default() -> LightVMCompileConfig {
  LightVMCompileConfig {
    target_arch: LIGHTVM_TARGET_ARCH_AARCH64,
    file_type: LIGHTVM_FILE_TYPE_BINARY,
    path: c"./bin/lightvm".as_ptr(),
  }
}
/// Compiles the loaded bytecode. A null `config` uses the defaults.
/// Returns the output path (free it with `lightvm_string_free`) or null on error.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_compile(vm: *mut LightVM, config: *const LightVMCompileConfig) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    let defaults = lightvm_compile_config_default();
    let config = if config.is_null() {
      &defaults
    } else {
      unsafe { &*config }
    };
    let result = read_c_str(config.path).and_then(|path| {
      let target_arch = match config.target_arch {
        LIGHTVM_TARGET_ARCH_AARCH64 => TargetArch::AArch64,
        _ => return Err(VMError::SystemError("Unknown target arch".into())),
      };
      let file_type = match config.file_type {
        LIGHTVM_FILE_TYPE_ASSEMBLY => FileType::Assembly,
        LIGHTVM_FILE_TYPE_BINARY => FileType::Binary,
        _ => return Err(VMError::SystemError("Unknown file type".into())),
      };
      let output_path = if matches!(file_type, FileType::Assembly) && !path.ends_with(".s") {
        format!("{}.s", path)
      } else {
        path.clone()
      };
      vm.compile_internal(CompileConfig {
        target_arch,
        file_type,
        path: path.as_str(),
      })
      .map_err(|error| VMError::SystemError(error.to_string().into()))?;
      Ok(output_path)
    });
    match result {
      Ok(path) => into_c_string(path),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Returns name, version, latest version and modules of the VM as JSON.
/// `modules` is the debug text of `ModuleVersions`.
#[cfg(not(feature = "wasm"))]
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_info(vm: *mut LightVM) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    let info = vm.info_internal();
    into_c_string(
      serde_json::json!({
        "name": info.name,
        "version": info.version,
        "latest_version": info.latest_version,
        "modules": format!("{:?}", info.modules),
      })
      .to_string(),
    )
  })
}
/// Creates a handle for an exported function or variable.
/// Free it with `lightvm_export_free` before freeing the VM.
///
/// # Examples
/// ```c
/// LightVMExport *add = lightvm_export(vm, "add");
/// char *out = lightvm_export_call(vm, add, "[5, 6]");
/// lightvm_string_free(out);
/// lightvm_export_free(add);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_export(vm: *mut LightVM, name: *const c_char) -> *mut LightVMExport {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    match read_c_str(name) {
      Ok(name) => Box::into_raw(Box::new(LightVMExport {
        is_function: vm.functions.contains_key(name.as_str()),
        name,
      })),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Returns 1 when the handle points to a function, 0 for a variable.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_export_is_function(handle: *const LightVMExport) -> c_int {
  if handle.is_null() {
    return LIGHTVM_ERR;
  }
  unsafe { &*handle }.is_function as c_int
}
/// Calls an exported function (`args_json` is a JSON array, null means no
/// arguments) or reads an exported variable (`args_json` is ignored).
/// Returns the raw JSON result or null on error.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_export_call(
  vm: *mut LightVM,
  handle: *const LightVMExport,
  args_json: *const c_char,
) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    if handle.is_null() {
      return fail(
        VMError::SystemError("Received a null export handle".into()),
        std::ptr::null_mut(),
      );
    }
    let handle = unsafe { &*handle };
    let result = if handle.is_function {
      read_args(args_json).and_then(|args| vm.call_exported_internal(handle.name.clone(), args))
    } else {
      vm.var_exported_internal(handle.name.clone())
    };
    match result {
      Ok(raw) => into_c_string(raw),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Frees a handle returned by `lightvm_export`.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_export_free(handle: *mut LightVMExport) {
  if !handle.is_null() {
    drop(unsafe { Box::from_raw(handle) });
  }
}
/// Optimizes raw JSON bytecode using the settings of the VM and returns it as
/// JSON text. Same as `tools().optimizeBytecode` in the TS interface.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_tools_optimize_bytecode(vm: *mut LightVM, input: *const c_char) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let Some(vm) = vm_mut(vm) else {
      return std::ptr::null_mut();
    };
    let result = read_json(input).and_then(|mut bytecode| {
      if let Some(raw) = bytecode.as_str() {
        bytecode = serde_json::from_str(raw).map_err(|error| {
          VMError::SystemError(format!("Failed to parse JSON string: {}", error).into())
        })?;
      }
      let tools = vm.tools();
      let mut caps = Vec::new();
      if tools.can_control {
        caps.push(Capability::Control);
      }
      if tools.can_debug {
        caps.push(Capability::Debug);
      }
      let config = VmConfig {
        caps,
        security_config: Some(SecurityConfig {
          time_budget: tools.time_budget,
          ..Default::default()
        }),
        runtime_config: Some(RuntimeConfig {
          nightly: tools.nightly,
        }),
        error_options: Some(ErrorOptions {
          backtrace: tools.backtrace,
          explain: tools.explain,
          hint: tools.hint,
          diagnostic_links: tools.diagnostic_links,
        }),
      };
      let mut optimizer = LightVM::new(config);
      optimizer
        .optimize_bytecode_internal(bytecode)
        .map_err(|error| VMError::SystemError(error.to_string().into()))
    });
    match result {
      Ok(text) => into_c_string(text),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Converts raw JSON bytecode into a readable LTC assembly string.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_tools_stringify_ltc(input: *const c_char) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    let result = read_json(input).and_then(LightVM::stringify_ltc_internal);
    match result {
      Ok(text) => into_c_string(unescape::unescape(&text).unwrap_or(text)),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Parses LTC code and returns the instructions as a JSON string.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_tools_parse_ltc(code: *const c_char) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    match read_c_str(code).and_then(LightVM::parse_ltc_internal) {
      Ok(text) => into_c_string(text),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
/// Parses an LTC string into a JSON array.
#[unsafe(no_mangle)]
pub extern "C" fn lightvm_tools_parse_ltc_array(code: *const c_char) -> *mut c_char {
  guard(std::ptr::null_mut(), || {
    match read_c_str(code).and_then(LightVM::parse_ltc_array_internal) {
      Ok(text) => into_c_string(text),
      Err(error) => fail(error, std::ptr::null_mut()),
    }
  })
}
#[cfg(test)]
mod tests {
  use super::*;
  fn c(text: &str) -> CString {
    CString::new(text).unwrap()
  }
  fn take(ptr: *mut c_char) -> String {
    assert!(!ptr.is_null());
    let text = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
    lightvm_string_free(ptr);
    text
  }
  fn vm_with(caps: &[c_int]) -> *mut LightVM {
    let mut config = lightvm_config_default();
    config.caps = caps.as_ptr();
    config.caps_len = caps.len();
    lightvm_new(&config)
  }
  #[test]
  fn default_config_matches_ts_defaults() {
    let config = lightvm_config_default();
    assert_eq!(config.caps_len, 1);
    assert_eq!(config.security_config.max_io, 100);
    assert_eq!(config.security_config.max_import, 3);
    assert_eq!(config.security_config.max_ticks, 1_000_000);
    assert!(config.error_options.hint);
    assert!(!config.runtime_config.nightly);
  }
  #[test]
  fn default_compile_config_matches_rust_defaults() {
    let config = lightvm_compile_config_default();
    assert_eq!(config.target_arch, LIGHTVM_TARGET_ARCH_AARCH64);
    assert_eq!(config.file_type, LIGHTVM_FILE_TYPE_BINARY);
    assert_eq!(read_c_str(config.path).unwrap(), "./bin/lightvm");
  }
  #[test]
  fn compile_rejects_unknown_file_type() {
    let vm = lightvm_new(std::ptr::null());
    let mut config = lightvm_compile_config_default();
    config.file_type = 99;
    assert!(lightvm_compile(vm, &config).is_null());
    lightvm_free(vm);
  }
  #[test]
  fn new_with_null_config_uses_defaults() {
    let vm = lightvm_new(std::ptr::null());
    assert!(!vm.is_null());
    assert_eq!(unsafe { &*vm }.max_ticks, 1_000_000);
    lightvm_free(vm);
    lightvm_free(std::ptr::null_mut());
  }
  #[test]
  fn new_rejects_unknown_capability() {
    assert!(vm_with(&[99]).is_null());
    assert!(!lightvm_last_error().is_null());
  }
  #[test]
  fn setters_chain_and_update_limits() {
    let vm = lightvm_new(std::ptr::null());
    let chained = lightvm_set_max_io(lightvm_set_max_ticks(vm, 10), 7);
    assert_eq!(chained, vm);
    assert_eq!(unsafe { &*vm }.max_ticks, 10);
    assert_eq!(unsafe { &*vm }.max_io, 7);
    assert!(lightvm_set_time_budget(vm, 99).is_null());
    lightvm_free(vm);
  }
  #[test]
  fn null_handle_stays_null_through_chain() {
    let vm = lightvm_set_max_io(std::ptr::null_mut(), 1);
    assert!(vm.is_null());
    assert!(lightvm_load(vm, c("[]").as_ptr()).is_null());
  }
  #[test]
  fn load_and_run_returns_json() {
    let vm = vm_with(&[LIGHTVM_CAPABILITY_OBSERVE, LIGHTVM_CAPABILITY_CONTROL]);
    assert_eq!(lightvm_load(vm, c(r#"[["push",42],["stop"]]"#).as_ptr()), vm);
    let options = LightVMRunOptions { capture_return: true };
    let raw = take(lightvm_run(vm, &options));
    assert!(raw.contains("42"));
    lightvm_free(vm);
  }
  #[test]
  fn export_calls_function() {
    let vm = vm_with(&[LIGHTVM_CAPABILITY_OBSERVE, LIGHTVM_CAPABILITY_CONTROL]);
    let source = r#"[
      ["jump", 7],
      ["func", "add", 2, 2, 6, "a", "b"],
      ["get", "a"],
      ["get", "b"],
      ["add", "int"],
      ["return"],
      ["stop"],
      ["export", "add"]
    ]"#;
    assert_eq!(lightvm_load(vm, c(source).as_ptr()), vm);
    lightvm_string_free(lightvm_run(vm, std::ptr::null()));
    let add = lightvm_export(vm, c("add").as_ptr());
    assert_eq!(lightvm_export_is_function(add), 1);
    let raw = take(lightvm_export_call(vm, add, c("[5, 6]").as_ptr()));
    assert!(raw.contains("11"));
    lightvm_export_free(add);
    lightvm_free(vm);
  }
  #[test]
  fn provide_variants_validate_json() {
    let vm = vm_with(&[LIGHTVM_CAPABILITY_CONTROL]);
    assert!(lightvm_provide(vm, c("name").as_ptr(), c("not json").as_ptr()).is_null());
    assert_eq!(lightvm_provide(vm, c("name").as_ptr(), c("\"John\"").as_ptr()), vm);
    assert_eq!(lightvm_provide_object(vm, c(r#"{"foo":1}"#).as_ptr()), vm);
    lightvm_free(vm);
  }
  #[test]
  fn paniclog_requires_debug_capability() {
    let vm = vm_with(&[LIGHTVM_CAPABILITY_OBSERVE]);
    assert!(lightvm_paniclog(vm).is_null());
    assert_eq!(lightvm_clear_paniclog(vm), LIGHTVM_ERR);
    lightvm_free(vm);
  }
}
