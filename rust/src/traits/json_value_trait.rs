/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[cfg(all(not(feature = "node"), not(feature = "wasm")))]
pub trait IntoJsonValue {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error>;
}
#[cfg(all(not(feature = "node"), not(feature = "wasm")))]
impl IntoJsonValue for &str {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(self).or_else(|_| Ok(serde_json::Value::String(self.to_string())))
  }
}
#[cfg(all(not(feature = "node"), not(feature = "wasm")))]
impl IntoJsonValue for String {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&self).or(Ok(serde_json::Value::String(self)))
  }
}
#[cfg(all(not(feature = "node"), not(feature = "wasm")))]
impl IntoJsonValue for serde_json::Value {
  fn into_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
    Ok(self)
  }
}
