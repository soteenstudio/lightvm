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
  #[ts(rename = "maxIo")]
  pub max_io: Option<u32>,
  #[ts(rename = "maxImport")]
  pub max_import: Option<u32>,
  #[ts(rename = "maxAlloc")]
  pub max_alloc: Option<u32>,
  #[ts(rename = "maxCall")]
  pub max_call: Option<u32>,
  #[ts(rename = "maxJump")]
  pub max_jump: Option<u32>,
  #[ts(rename = "maxTicks")]
  pub max_ticks: Option<u32>,
  #[ts(rename = "maxStackSize")]
  pub max_stack_size: Option<u32>,
  #[ts(rename = "allowedImports")]
  pub allowed_imports: Option<Vec<String>>,
  #[ts(rename = "unsafeMode")]
  pub unsafe_mode: Option<bool>,
}
