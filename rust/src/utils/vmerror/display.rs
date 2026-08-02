/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::vmerror::colors::*;
use crate::utils::vmerror::config::get_thread_or_global_config;
use crate::utils::vmerror::error::VMError;
use crate::utils::vmerror::get_backtrace::get_backtrace;
use crate::utils::vmerror::hints::get_hint;
use smol_str::SmolStr;
use std::fmt;
impl fmt::Display for VMError {
  #[cold]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let config = get_thread_or_global_config();
    let is_backtrace = config.backtrace;
    let is_explain = config.explain;
    let is_hint = config.hint;
    let err_type = match self {
      VMError::StackOverflow { .. } => "StackOverflow",
      VMError::StackUnderflow { .. } => "StackUnderflow",
      VMError::InvalidOpcode { .. } => "InvalidOpcode",
      VMError::TypeMismatch { .. } => "TypeMismatch",
      VMError::OutOfBounds { .. } => "OutOfBounds",
      VMError::InvalidJumpTarget { .. } => "InvalidJumpTarget",
      VMError::FeatureRestricted { .. } => "FeatureRestricted",
      VMError::IoFlood { .. } => "IoFlood",
      VMError::ImportLimitReached { .. } => "ImportLimitReached",
      VMError::UnauthorizedModule { .. } => "UnauthorizedModule",
      VMError::MemoryLimitExceeded { .. } => "MemoryLimitExceeded",
      VMError::CallLimitExceeded { .. } => "CallLimitExceeded",
      VMError::JumpLimitExceeded { .. } => "JumpLimitExceeded",
      VMError::ExcessiveNopPadding => "ExcessiveNopPadding",
      VMError::InvalidMaxTicksConfig => "InvalidMaxTicksConfig",
      VMError::TickLimitExceeded => "TickLimitExceeded",
      VMError::SystemError(_) => "SystemError",
    };
    let ip = match self {
      VMError::SystemError(_)
      | VMError::ExcessiveNopPadding
      | VMError::InvalidMaxTicksConfig
      | VMError::TickLimitExceeded => 0,
      VMError::StackOverflow { ip, .. }
      | VMError::StackUnderflow { ip, .. }
      | VMError::InvalidOpcode { ip, .. }
      | VMError::TypeMismatch { ip, .. }
      | VMError::OutOfBounds { ip, .. }
      | VMError::InvalidJumpTarget { ip, .. }
      | VMError::FeatureRestricted { ip, .. }
      | VMError::IoFlood { ip }
      | VMError::ImportLimitReached { ip }
      | VMError::UnauthorizedModule { ip, .. }
      | VMError::MemoryLimitExceeded { ip }
      | VMError::CallLimitExceeded { ip }
      | VMError::JumpLimitExceeded { ip } => *ip,
    };
    write!(f, "{BOLD}{RED}Error[{}]{RESET}: ", self.error_code())?;
    match self {
      VMError::StackOverflow { limit, .. } => write!(f, "Stack limit reached (limit: {}).", limit),
      VMError::StackUnderflow { opcode, .. } => write!(
        f,
        "Attempted to pop from an empty stack during {BOLD}'{}' {RESET}instruction.",
        opcode,
      ),
      VMError::InvalidOpcode { code, .. } => {
        write!(
          f,
          "Illegal instruction {BOLD}'{}' {RESET}encountered.",
          code
        )
      }
      VMError::TypeMismatch {
        expected, found, ..
      } => write!(
        f,
        "Type mismatch. Expected type '{}', but found '{}'.",
        expected, found
      ),
      VMError::OutOfBounds { index, len, .. } => write!(
        f,
        "Index out of bounds. Accessing index {} on a collection of length {}.",
        index, len
      ),
      VMError::InvalidJumpTarget { target, len, .. } => write!(
        f,
        "Invalid jump target {} (Bytecode length is {}).",
        target, len
      ),
      VMError::FeatureRestricted { feature, .. } => {
        write!(
          f,
          "The feature/opcode {BOLD}'{}' {RESET}is restricted.",
          feature
        )
      }
      VMError::IoFlood { .. } => {
        write!(f, "Maximum allowed I/O operations reached")
      }
      VMError::ImportLimitReached { .. } => {
        write!(f, "Maximum allowed module imports reached")
      }
      VMError::UnauthorizedModule { module, .. } => {
        write!(f, "Import of module '{}' is unauthorized", module)
      }
      VMError::MemoryLimitExceeded { .. } => {
        write!(f, "Maximum allowed memory allocation reached")
      }
      VMError::CallLimitExceeded { .. } => {
        write!(f, "Maximum allowed function calls reached")
      }
      VMError::JumpLimitExceeded { .. } => {
        write!(f, "Maximum allowed control flow jumps reached")
      }
      VMError::ExcessiveNopPadding => {
        write!(f, "Nop instruction padding exceeds permitted limit")
      }
      VMError::InvalidMaxTicksConfig => {
        write!(
          f,
          "Invalid max_ticks configuration. value must be greater than zero"
        )
      }
      VMError::TickLimitExceeded => {
        write!(f, "Maximum allowed execution ticks reached")
      }
      VMError::SystemError(s) => write!(f, "{}", s),
    }?;
    if !matches!(self, VMError::SystemError(_)) {
      if is_hint {
        write!(
          f,
          "\n {RESET}{CYAN}│   {DARK_GRAY}at instruction pointer: {ip}{RESET}"
        )?;
        write!(f, "\n {RESET}{CYAN}│   {DARK_GRAY}error type: {}", err_type)?;
      } else {
        write!(f, "\n     {DARK_GRAY}at instruction pointer: {ip}{RESET}")?;
        write!(f, "\n     {DARK_GRAY}error type: {}", err_type)?;
      }
    }
    if is_backtrace {
      let backtrace = get_backtrace();
      write!(
        f,
        "\n {RESET}{CYAN}│   {DARK_GRAY}internal backtrace:\n{backtrace}{RESET}"
      )?;
    }
    if let Some(hint_data) = get_hint(self) {
      if is_hint {
        let text = if is_explain {
          hint_data.long
        } else {
          hint_data.short
        };
        let section = if is_explain {
          "hint (explained): "
        } else {
          "hint: "
        };
        write!(
          f,
          "\n {RESET}{CYAN}│\n {CYAN}└─ {CYAN}{section}{DARK_GRAY}{text}{RESET}\n\n"
        )?
      } else {
        write!(f, "{RESET}\n\n")?;
      }
    }
    Ok(())
  }
}
impl From<VMError> for SmolStr {
  fn from(err: VMError) -> Self {
    SmolStr::from(err.to_string())
  }
}
