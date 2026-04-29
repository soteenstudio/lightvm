/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use napi_derive::napi;
use serde::{Deserialize, Serialize};
#[napi]
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Capability {
  Control,
  Observe,
  Debug,
  Unsafe,
}
impl Capability {
  pub fn from_str(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "control" => Some(Self::Control),
      "observe" => Some(Self::Observe),
      "debug" => Some(Self::Debug),
      "unsafe" => Some(Self::Unsafe),
      _ => None,
    }
  }
}
