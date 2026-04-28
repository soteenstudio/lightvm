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
#[napi(string_enum)]
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Capability {
  Control,
  Observe,
  Debug,
  Unsafe,
}
