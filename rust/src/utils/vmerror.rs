/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use smol_str::SmolStr;
pub enum VMError {
  /// Terjadi saat stack mencapai batas maksimum yang ditentukan InitStack atau default
  StackOverflow { ip: usize, limit: usize },
  /// Terjadi saat opcode mencoba pop data tapi stack kosong
  StackUnderflow { ip: usize, opcode: &'static str },
  /// Terjadi saat parser atau executor nemu bytecode ilegal
  InvalidOpcode { ip: usize, code: SmolStr },
  /// Terjadi saat operasi (misal: Add) dapet tipe data yang gak sinkron
  TypeMismatch {
    ip: usize,
    expected: &'static str,
    found: &'static str,
  },
  /// Error umum terkait environment atau OS
  SystemError(SmolStr),
  /// Error saat akses index di luar jangkauan (Array/Object)
  OutOfBounds { ip: usize, index: usize, len: usize },
}
impl VMError {
  /// Mengembalikan kode error unik untuk dokumentasi (misal: LVM001)
  fn error_code(&self) -> &'static str {
    match self {
      VMError::StackOverflow { .. } => "LVM001",
      VMError::StackUnderflow { .. } => "LVM002",
      VMError::InvalidOpcode { .. } => "LVM003",
      VMError::TypeMismatch { .. } => "LVM004",
      VMError::OutOfBounds { .. } => "LVM005",
      VMError::SystemError(_) => "LVM500",
    }
  }
  pub fn format(&self) -> SmolStr {
    let red = "\x1b[31;1m";
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";
    let code = self.error_code();
    let prefix = format!("{}[LightVM]{}", bold, reset);
    let detail = match self {
      VMError::StackOverflow { ip, limit } =>
          format!("Stack limit reached (limit: {}) at [IP: {}]. Potential infinite recursion or unoptimized InitStack.", limit, ip),
      VMError::StackUnderflow { ip, opcode } =>
          format!("Attempted to pop from an empty stack during '{}' instruction at [IP: {}].", opcode, ip),
      VMError::InvalidOpcode { ip, code } =>
          format!("Illegal instruction '{}' encountered at [IP: {}]. Bytecode may be corrupted.", code, ip),
      VMError::TypeMismatch { ip, expected, found } =>
          format!("Type mismatch at [IP: {}]. Expected type '{}', but found '{}'.", ip, expected, found),
      VMError::OutOfBounds { ip, index, len } =>
          format!("Index out of bounds at [IP: {}]. Accessing index {} on a collection of length {}.", ip, index, len),
      VMError::SystemError(s) => s.to_string(),
    };
    SmolStr::new(format!(
      "{prefix} {red}Runtime Error {code}{reset}: {detail}\n{yellow}Location: {cyan}instruction_pointer: {ip}{reset}",
      prefix = prefix,
      red = red,
      code = code,
      reset = reset,
      detail = detail,
      yellow = yellow,
      cyan = cyan,
      ip = match self {
          VMError::SystemError(_) => 0,
          VMError::StackOverflow { ip, .. } | VMError::StackUnderflow { ip, .. } |
          VMError::InvalidOpcode { ip, .. } | VMError::TypeMismatch { ip, .. } |
          VMError::OutOfBounds { ip, .. } => *ip,
      }
    ))
  }
}
