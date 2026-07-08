/*
 * Copyright 2026 SoTeen Studio
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
#[ts(export, rename = "ErrorOptions")]
pub struct JSErrorOptions {
  #[ts(type = "boolean")]
  pub backtrace: Option<bool>,
  #[ts(type = "boolean")]
  pub explain: Option<bool>,
  #[ts(type = "boolean")]
  pub hint: Option<bool>,
}
