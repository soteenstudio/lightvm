/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{file_type::FileType, target_arch::TargetArch};
#[derive(Debug, Clone, Copy, ts_rs::TS)]
#[ts(export, rename = "CompileConfig")]
pub struct CompileConfig<'a> {
  #[ts(rename = "targetArch", type = "number")]
  pub target_arch: TargetArch,
  #[ts(rename = "fileType", type = "number")]
  pub file_type: FileType,
  #[ts(type = "string")]
  pub path: &'a str,
}
impl<'a> Default for CompileConfig<'a> {
  fn default() -> Self {
    Self {
      target_arch: TargetArch::AArch64,
      file_type: FileType::Binary,
      path: "./bin/lightvm",
    }
  }
}
