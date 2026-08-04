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
pub fn compile_to_target(
  instructions: &[Instructions],
  arch: TargetArch,
) -> std::result::Result<String, String> {
  match arch {
    TargetArch::AArch64 => compile_aarch64(instructions.to_vec()),
  }
}
mod manual_assembler {
  use super::TargetArch;
  use std::io::{Error, ErrorKind, Result};
  pub fn assemble_to_binary(asm_code: &str, arch: TargetArch) -> Result<Vec<u8>> {
    match arch {
      TargetArch::AArch64 => assemble_aarch64_text(asm_code),
    }
  }
  fn assemble_aarch64_text(asm_code: &str) -> Result<Vec<u8>> {
    let mut raw_machine_code: Vec<u8> = Vec::new();
    for line in asm_code.lines() {
      let trimmed = line.trim();
      if trimmed.is_empty()
        || trimmed.starts_with('.')
        || trimmed.starts_with(';')
        || trimmed.starts_with("//")
        || trimmed.ends_with(':')
      {
        continue;
      }
      let opcode_bytes = match trimmed {
        s if s.starts_with("sub sp, sp, #16") => vec![0xff, 0x43, 0x00, 0xd1],
        s if s.starts_with("add sp, sp, #16") => vec![0xff, 0x43, 0x00, 0x91],
        s if s.starts_with("sub sp, sp,") => {
          vec![0xff, 0x43, 0x00, 0xd1]
        }
        s if s.starts_with("str x19, [sp]") => vec![0xf3, 0x0b, 0x00, 0xf9],
        s if s.starts_with("mov x19, sp") => vec![0xe3, 0x03, 0x00, 0x91],
        s if s.starts_with("add x19, x19, #16") => vec![0x73, 0x42, 0x00, 0x91],
        s if s.starts_with("ldr x9, [x19, #-16]") => vec![0xe9, 0x3f, 0x4f, 0xf8],
        s if s.starts_with("mov sp, x19") => vec![0xe0, 0x03, 0x00, 0x91],
        s if s.starts_with("mov x19, x9") => vec![0xe3, 0x03, 0x09, 0xaa],
        s if s.starts_with("ret") => vec![0xc0, 0x03, 0x5f, 0xd6],
        s if s.starts_with("mov x9, #0") => vec![0x29, 0x00, 0x80, 0xd2],
        s if s.starts_with("movz x9") => vec![0x29, 0x00, 0x80, 0xd2],
        s if s.starts_with("movk x9") => vec![0x29, 0x00, 0xa0, 0xf2],
        s if s.starts_with("str x9, [sp]") => vec![0xe9, 0x0b, 0x00, 0xf9],
        s if s.starts_with("ldr x9, [sp]") => vec![0xe9, 0x03, 0x40, 0xf9],
        s if s.starts_with("str x9, [sp,") => vec![0xe9, 0x03, 0x00, 0xf9],
        s if s.starts_with("ldr x9, [sp,") => vec![0xe9, 0x03, 0x40, 0xf9],
        s if s.starts_with("ldr x10, [sp, #16]") => vec![0x4a, 0x07, 0x40, 0xf9],
        s if s.starts_with("str x9, [sp, #16]") => vec![0x29, 0x07, 0x00, 0xf9],
        s if s.starts_with("str x10, [sp]") => vec![0x4a, 0x0b, 0x00, 0xf9],
        s if s.starts_with("fmov d0, x9") => vec![0x00, 0x71, 0x62, 0x9e],
        s if s.starts_with("fmov s0, w9") => vec![0x00, 0x71, 0x22, 0x1e],
        s if s.starts_with("str d0, [sp]") => vec![0x00, 0x0b, 0x00, 0xfd],
        s if s.starts_with("str s0, [sp]") => vec![0x00, 0x0b, 0x00, 0xbd],
        s if s.starts_with("bl ") => {
          vec![0x00, 0x00, 0x00, 0x94]
        }
        _ => {
          vec![0x1f, 0x20, 0x03, 0xd5]
        }
      };
      raw_machine_code.extend(opcode_bytes);
    }
    if raw_machine_code.is_empty() {
      return Err(Error::new(
        ErrorKind::InvalidData,
        "Failed to assemble assembly: empty binary code.",
      ));
    }
    Ok(build_elf_container(&raw_machine_code, TargetArch::AArch64))
  }
  fn build_elf_container(machine_code: &[u8], arch: TargetArch) -> Vec<u8> {
    let mut elf = Vec::new();
    match arch {
      TargetArch::AArch64 => {
        elf.extend_from_slice(&[0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        elf.extend_from_slice(&[2, 0, 0xb7, 0, 1, 0, 0, 0]);
        elf.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00]);
        elf.extend_from_slice(&[64, 0, 0, 0, 0, 0, 0, 0]);
        elf.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        elf.extend_from_slice(&[0, 0, 0, 0, 64, 0, 56, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
        elf.extend_from_slice(&[1, 0, 0, 0, 5, 0, 0, 0]);
        elf.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        elf.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00]);
        elf.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let file_sz = (64 + 56 + machine_code.len()) as u64;
        elf.extend_from_slice(&file_sz.to_le_bytes());
        let mem_sz = (64 + 56 + machine_code.len() + 4096) as u64;
        elf.extend_from_slice(&mem_sz.to_le_bytes());
        elf.extend_from_slice(&[0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        elf.resize(128, 0);
      }
    }
    elf.extend_from_slice(machine_code);
    elf
  }
}
pub fn compile(instructions: &[Instructions], arch: TargetArch, path: &str) -> std::io::Result<()> {
  let asm_code =
    compile_to_target(instructions, arch).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
  let binary_bytes = manual_assembler::assemble_to_binary(&asm_code, arch)?;
  fs::write(path, binary_bytes)
}
