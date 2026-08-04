/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::codegen::arch::aarch64::compile_aarch64;
use crate::types::instructions::Instructions;
use crate::types::target_arch::TargetArch;
use std::io::Result;
pub fn compile_to_target(instructions: &[Instructions], arch: TargetArch) -> Result<String, String> {
  match arch {
    TargetArch::AArch64 => compile_aarch64(instructions.to_vec()),
  }
}
pub fn compile(instructions: &[Instructions], arch: TargetArch, path: &str) -> Result<()> {
  let asm_code = compile_to_target(instructions, arch)
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
  std::fs::write(path, asm_code)
}
