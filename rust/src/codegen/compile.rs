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
use std::fs;
use std::io::{Error, ErrorKind};
use std::process::Command;

const IO_C_CONTENT: &str = include_str!("../../../libs/io.c");

pub fn compile_to_target(
  instructions: &[Instructions],
  arch: TargetArch,
) -> std::result::Result<String, String> {
  match arch {
    TargetArch::AArch64 => compile_aarch64(instructions.to_vec()),
  }
}

fn check_tool_exists(tool: &str) -> bool {
  Command::new(tool)
    .arg("--version")
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}

pub fn compile(instructions: &[Instructions], arch: TargetArch, path: &str) -> std::io::Result<()> {
  let asm_code =
    compile_to_target(instructions, arch).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

  let asm_path = format!("{}.s", path);
  fs::write(&asm_path, &asm_code)?;

  let io_c_path = format!("{}_io.c", path);
  fs::write(&io_c_path, IO_C_CONTENT)?;

  let compiler = if check_tool_exists("clang") {
    "clang"
  } else if check_tool_exists("gcc") {
    "gcc"
  } else {
    let _ = fs::remove_file(&asm_path);
    let _ = fs::remove_file(&io_c_path);
    return Err(Error::new(
      ErrorKind::NotFound,
      "No C compiler found on your system! Please install 'clang' or 'gcc'.",
    ));
  };

  let status = Command::new(compiler)
    .arg(&io_c_path)
    .arg(&asm_path)
    .arg("-O2")
    .arg("-nostartfiles")
    .arg("-o")
    .arg(path)
    .status();

  let _ = fs::remove_file(&asm_path);
  let _ = fs::remove_file(&io_c_path);

  match status {
    Ok(s) if s.success() => Ok(()),
    Ok(s) => Err(Error::other(format!(
      "Compilation failed with exit code: {}",
      s
    ))),
    Err(e) => Err(Error::other(format!(
      "Failed to execute compiler command: {}",
      e
    ))),
  }
}
