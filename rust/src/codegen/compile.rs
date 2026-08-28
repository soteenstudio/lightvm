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
use crate::modules::vmerror::VMError;
use crate::types::instructions::Instructions;
use crate::types::{file_type::FileType, target_arch::TargetArch};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
const IO_C_CONTENT: &str = include_str!("../../../libs/io.c");
static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);
fn get_ram_dir() -> String {
  if (cfg!(target_os = "linux") || cfg!(target_os = "android"))
    && std::path::Path::new("/dev/shm").exists()
  {
    return "/dev/shm".to_string();
  }
  std::env::temp_dir().to_string_lossy().into_owned()
}
fn create_secure_temp_dir() -> std::result::Result<PathBuf, VMError> {
  let base_dir = PathBuf::from(get_ram_dir());
  let pid = std::process::id();
  for _ in 0..100 {
    let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .map(|d| d.subsec_nanos())
      .unwrap_or(0);
    let dir_name = format!("lightvm_{}_{}_{}_{}", pid, counter, nanos, fastrand());
    let temp_dir = base_dir.join(dir_name);
    #[cfg(unix)]
    {
      use std::os::unix::fs::DirBuilderExt;
      let mut builder = fs::DirBuilder::new();
      builder.mode(0o700);
      match builder.create(&temp_dir) {
        Ok(()) => return Ok(temp_dir),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
        Err(e) => return Err(VMError::SystemError(e.to_string().into())),
      }
    }
    #[cfg(not(unix))]
    {
      match fs::create_dir(&temp_dir) {
        Ok(()) => return Ok(temp_dir),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
        Err(e) => return Err(VMError::SystemError(e.to_string().into())),
      }
    }
  }
  Err(VMError::SystemError(
    "Failed to create unique temporary directory after 100 attempts".into(),
  ))
}
fn fastrand() -> u64 {
  let pid = std::process::id() as u64;
  let time = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .map(|d| d.as_nanos() as u64)
    .unwrap_or(0);
  pid
    .wrapping_mul(6364136223846793005)
    .wrapping_add(time)
    .wrapping_mul(1442695040888963407)
}
pub fn compile_to_target(
  instructions: &[Instructions],
  arch: TargetArch,
) -> std::result::Result<String, VMError> {
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
pub fn compile(
  instructions: &[Instructions],
  arch: TargetArch,
  path: &str,
  file_type: FileType,
) -> std::result::Result<(), VMError> {
  let asm_code = compile_to_target(instructions, arch)?;
  if matches!(file_type, FileType::Assembly) {
    let asm_path = if path.ends_with(".s") {
      path.to_string()
    } else {
      format!("{}.s", path)
    };
    if let Some(parent) = std::path::Path::new(&asm_path).parent()
      && !parent.as_os_str().is_empty()
    {
      match fs::create_dir_all(parent) {
        Ok(()) => {}
        Err(e) => return Err(VMError::SystemError(e.to_string().into())),
      }
    }
    match fs::write(&asm_path, &asm_code) {
      Ok(()) => return Ok(()),
      Err(e) => return Err(VMError::SystemError(e.to_string().into())),
    }
  }
  if let Some(parent) = std::path::Path::new(path).parent()
    && !parent.as_os_str().is_empty()
  {
    match fs::create_dir_all(parent) {
      Ok(()) => {}
      Err(e) => return Err(VMError::SystemError(e.to_string().into())),
    }
  }
  let compiler = match arch {
    TargetArch::AArch64 => {
      if check_tool_exists("clang") {
        "clang"
      } else if check_tool_exists("gcc") {
        "gcc"
      } else {
        return Err(VMError::SystemError(
          "No C compiler found on your system! Please install 'clang' or 'gcc'.".into(),
        ));
      }
    }
  };
  let temp_dir = create_secure_temp_dir()?;
  let ram_asm_path = temp_dir.join("lightvm.s");
  let ram_c_path = temp_dir.join("lightvm_io.c");
  let mut asm_file = match OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(&ram_asm_path)
  {
    Ok(file) => file,
    Err(e) => {
      let _ = fs::remove_dir_all(&temp_dir);
      return Err(VMError::SystemError(e.to_string().into()));
    }
  };
  match asm_file.write_all(asm_code.as_bytes()) {
    Ok(()) => {}
    Err(e) => {
      drop(asm_file);
      let _ = fs::remove_dir_all(&temp_dir);
      return Err(VMError::SystemError(e.to_string().into()));
    }
  }
  drop(asm_file);
  let mut c_file = match OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(&ram_c_path)
  {
    Ok(file) => file,
    Err(e) => {
      let _ = fs::remove_dir_all(&temp_dir);
      return Err(VMError::SystemError(e.to_string().into()));
    }
  };
  match c_file.write_all(IO_C_CONTENT.as_bytes()) {
    Ok(()) => {}
    Err(e) => {
      drop(c_file);
      let _ = fs::remove_dir_all(&temp_dir);
      return Err(VMError::SystemError(e.to_string().into()));
    }
  }
  drop(c_file);
  let status = Command::new(compiler)
    .arg(&ram_c_path)
    .arg(&ram_asm_path)
    .arg("-O2")
    .arg("-o")
    .arg(path)
    .status();
  let _ = fs::remove_dir_all(&temp_dir);
  match status {
    Ok(s) if s.success() => Ok(()),
    Ok(s) => {
      let msg = match s.code() {
        Some(code) => format!("Compilation failed with exit code: {}", code),
        None => "Compilation failed: process terminated by signal".to_string(),
      };
      Err(VMError::SystemError(msg.into()))
    }
    Err(e) => Err(VMError::SystemError(
      format!("Failed to execute compiler command: {}", e).into(),
    )),
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn embedded_c_runtime_formats_boolean_tag_as_true_false() {
    assert!(IO_C_CONTENT.contains("type_tag == 1"));
    assert!(IO_C_CONTENT.contains("\"true\""));
    assert!(IO_C_CONTENT.contains("\"false\""));
    assert!(IO_C_CONTENT.contains("lightvm_stdout(LightVMValue"));
    assert!(IO_C_CONTENT.contains("lightvm_stdoutln(LightVMValue"));
  }
}
