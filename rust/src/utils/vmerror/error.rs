/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use smol_str::SmolStr;
use std::borrow::Cow;
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
  /// Occurs when a nightly/experimental opcode is used but nightly mode is disabled.
  FeatureRestricted { ip: usize, feature: &'static str },
}
pub struct Hint {
  pub short: Cow<'static, str>,
  pub long: Cow<'static, str>,
}
impl VMError {
  /// Returns a unique error code for documentation (e.g., LVM001)
  #[cold]
  pub fn error_code(&self) -> &'static str {
    match self {
      VMError::StackOverflow { .. } => "LVM001",
      VMError::StackUnderflow { .. } => "LVM002",
      VMError::InvalidOpcode { .. } => "LVM003",
      VMError::TypeMismatch { .. } => "LVM004",
      VMError::OutOfBounds { .. } => "LVM005",
      VMError::InvalidJumpTarget { .. } => "LVM006",
      VMError::FeatureRestricted { .. } => "LVM007",
      VMError::SystemError(_) => "LVM500",
    }
  }
}
