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
use crate::types::js::js_module_versions::JSModuleVersions;
use napi_derive::napi;
#[napi(object)]
pub struct JSInfoVM {
  pub name: String,
  pub version: String,
  pub latest_version: Option<String>,
  pub modules: JSModuleVersions,
}
