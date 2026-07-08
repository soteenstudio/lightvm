/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::capability::Capability;
use crate::types::error_options::ErrorOptions;
#[cfg(feature = "node")]
use crate::types::js_error_options::JSErrorOptions;
#[cfg(feature = "node")]
use crate::types::js_runtime_config::JSRuntimeConfig;
use crate::types::runtime_config::RuntimeConfig;
#[cfg(feature = "node")]
use napi_derive::napi;
#[derive(Default)]
pub struct VmConfig {
  pub caps: Vec<Capability>,
  pub runtime_config: Option<RuntimeConfig>,
  pub error_options: Option<ErrorOptions>,
}
#[cfg(feature = "node")]
#[napi(object)]
#[derive(Default, ts_rs::TS)]
#[ts(export, rename = "VMConfig")]
pub struct VmNapiConfig {
  #[ts(rename = "caps")]
  pub caps_raw: Vec<u32>,
  #[ts(rename = "errorOptions")]
  pub error_options: Option<JSErrorOptions>,
  #[ts(rename = "runtimeConfig")]
  pub runtime_config: Option<JSRuntimeConfig>,
}
#[cfg(feature = "wasm")]
#[derive(serde::Deserialize)]
pub struct VmWasmConfig {
  pub caps: Vec<u32>,
  pub nightly: Option<bool>,
  pub backtrace: Option<bool>,
  pub explain: Option<bool>,
  pub hint: Option<bool>,
}
