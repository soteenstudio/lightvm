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
use napi_derive::napi;
#[derive(Default)]
pub struct VmConfig {
  pub caps: Vec<Capability>,
  pub error_options: ErrorOptions,
}
#[cfg(feature = "node")]
#[napi(object)]
#[derive(Default, ts_rs::TS)]
#[ts(export, rename = "VMConfig")]
pub struct VmNapiConfig {
  #[ts(rename = "caps")]
  pub caps_raw: Vec<u32>,
  pub nightly: Option<bool>,
  pub backtrace: Option<bool>,
  pub explain: Option<bool>,
  pub hint: Option<bool>,
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
