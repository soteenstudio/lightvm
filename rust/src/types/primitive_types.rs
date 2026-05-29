/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PrimitiveTypes {
  Sht,
  Int,
  Lng,
  Oct,
  Hlf,
  Flt,
  Dbl,
  Str,
}
