/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[cfg(feature = "node")]
use napi_derive::napi;
#[cfg(feature = "node")]
#[napi(object)]
#[derive(Default, ts_rs::TS)]
#[ts(export, rename = "SecurityConfig")]
pub struct JSSecurityConfig {
  pub max_io: Option<u32>,
  pub max_import: Option<u32>,
  pub max_alloc: Option<u32>,
  pub max_call: Option<u32>,
  pub max_jump: Option<u32>,
  pub allowed_imports: Option<Vec<String>>,
  pub unsafe_mode: Option<bool>,
}
