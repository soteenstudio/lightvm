/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use smol_str::SmolStr;
use std::fmt;
#[derive(Debug)]
pub enum VMError {
  /// Occurs when the stack reaches the maximum limit specified by InitStack or default.
  StackOverflow { ip: usize, limit: usize },
  /// Occurs when the opcode tries to pop data but the stack is empty.
  StackUnderflow { ip: usize, opcode: &'static str },
  /// Occurs when the parser or executor encounters illegal bytecode.
  InvalidOpcode { ip: usize, code: SmolStr },
  /// Occurs when an operation (e.g. Add) encounters an asynchronous data type.
  TypeMismatch {
    ip: usize,
    expected: &'static str,
    found: &'static str,
  },
  /// Common errors related to the environment or OS
  SystemError(SmolStr),
  /// Error when accessing index out of range (Array/Object)
  OutOfBounds { ip: usize, index: usize, len: usize },
  /// Occurs when control flow jumps to an IP that is outside the bytecode length.
  InvalidJumpTarget {
    ip: usize,
    target: usize,
    len: usize,
  },
}
impl fmt::Display for VMError {
  #[cold]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let red = "\x1b[31;1m";
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";
    write!(
      f,
      "{bold}[LightVM]{reset} {red}Runtime Error {}{reset}: ",
      self.error_code()
    )?;
    match self {
      VMError::StackOverflow { ip, limit } => write!(
        f,
        "Stack limit reached (limit: {}) at [IP: {}]. Potential infinite recursion or unoptimized InitStack.",
        limit, ip
      ),
      VMError::StackUnderflow { ip, opcode } => write!(
        f,
        "Attempted to pop from an empty stack during '{}' instruction at [IP: {}].",
        opcode, ip
      ),
      VMError::InvalidOpcode { ip, code } => write!(
        f,
        "Illegal instruction '{}' encountered at [IP: {}]. Bytecode may be corrupted.",
        code, ip
      ),
      VMError::TypeMismatch {
        ip,
        expected,
        found,
      } => write!(
        f,
        "Type mismatch at [IP: {}]. Expected type '{}', but found '{}'.",
        ip, expected, found
      ),
      VMError::OutOfBounds { ip, index, len } => write!(
        f,
        "Index out of bounds at [IP: {}]. Accessing index {} on a collection of length {}.",
        ip, index, len
      ),
      VMError::InvalidJumpTarget { ip, target, len } => write!(
        f,
        "Invalid jump target {} at [IP: {}]. Bytecode length is only {}. Execution aborted to prevent UB.",
        target, ip, len
      ),
      VMError::SystemError(s) => write!(f, "{}", s),
    }?;
    let ip = match self {
      VMError::SystemError(_) => 0,
      _ => *match self {
        VMError::StackOverflow { ip, .. }
        | VMError::StackUnderflow { ip, .. }
        | VMError::InvalidOpcode { ip, .. }
        | VMError::TypeMismatch { ip, .. }
        | VMError::OutOfBounds { ip, .. }
        | VMError::InvalidJumpTarget { ip, .. } => ip,
        _ => &0,
      },
    };
    write!(
      f,
      "\n{yellow}Location: {cyan}instruction_pointer: {ip}{reset}"
    )
  }
}
impl From<VMError> for SmolStr {
  fn from(err: VMError) -> Self {
    SmolStr::from(err.to_string())
  }
}
impl VMError {
  /// Returns a unique error code for documentation (e.g., LVM001)
  #[cold]
  fn error_code(&self) -> &'static str {
    match self {
      VMError::StackOverflow { .. } => "LVM001",
      VMError::StackUnderflow { .. } => "LVM002",
      VMError::InvalidOpcode { .. } => "LVM003",
      VMError::TypeMismatch { .. } => "LVM004",
      VMError::OutOfBounds { .. } => "LVM005",
      VMError::InvalidJumpTarget { .. } => "LVM006",
      VMError::SystemError(_) => "LVM500",
    }
  }
}
