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
  StackOverflow {
    ip: usize,
    limit: usize,
  },
  /// Occurs when the opcode tries to pop data but the stack is empty.
  StackUnderflow {
    ip: usize,
    opcode: &'static str,
  },
  /// Occurs when the parser or executor encounters illegal bytecode.
  InvalidOpcode {
    ip: usize,
    code: SmolStr,
  },
  /// Occurs when an operation (e.g. Add) encounters an asynchronous data type.
  TypeMismatch {
    ip: usize,
    expected: &'static str,
    found: &'static str,
  },
  /// Common errors related to the environment or OS
  SystemError(SmolStr),
  /// Error when accessing index out of range (Array/Object)
  OutOfBounds {
    ip: usize,
    index: usize,
    len: usize,
  },
  /// Occurs when control flow jumps to an IP that is outside the bytecode length.
  InvalidJumpTarget {
    ip: usize,
    target: usize,
    len: usize,
  },
  /// Occurs when a nightly/experimental opcode is used but nightly mode is disabled.
  FeatureRestricted {
    ip: usize,
    feature: &'static str,
  },
  IoFlood {
    ip: usize,
  },
  ImportLimitReached {
    ip: usize,
  },
  UnauthorizedModule {
    ip: usize,
    module: SmolStr,
  },
  MemoryLimitExceeded {
    ip: usize,
  },
  CallLimitExceeded {
    ip: usize,
  },
  JumpLimitExceeded {
    ip: usize,
  },
  ExcessiveNopPadding,
  InvalidMaxTicksConfig,
  TickLimitExceeded,
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
      VMError::IoFlood { .. } => "LVM008",
      VMError::ImportLimitReached { .. } => "LVM009",
      VMError::UnauthorizedModule { .. } => "LVM010",
      VMError::MemoryLimitExceeded { .. } => "LVM011",
      VMError::CallLimitExceeded { .. } => "LVM012",
      VMError::JumpLimitExceeded { .. } => "LVM013",
      VMError::ExcessiveNopPadding => "LVM014",
      VMError::InvalidMaxTicksConfig => "LVM015",
      VMError::TickLimitExceeded => "LVM016",
      VMError::SystemError(_) => "LVM500",
    }
  }

  /// Returns the documentation URL for this error.
  #[cold]
  pub fn diagnostic_link(&self) -> String {
    format!(
      "https://lightvm.vercel.app/api-reference/error-codes/{}-code",
      self.error_code().to_ascii_lowercase()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::VMError;
  use crate::modules::vmerror::config::set_thread_error_config;
  use smol_str::SmolStr;

  fn all_errors() -> Vec<VMError> {
    vec![
      VMError::StackOverflow { ip: 1, limit: 2 },
      VMError::StackUnderflow {
        ip: 1,
        opcode: "POP",
      },
      VMError::InvalidOpcode {
        ip: 1,
        code: SmolStr::new("INVALID"),
      },
      VMError::TypeMismatch {
        ip: 1,
        expected: "number",
        found: "string",
      },
      VMError::SystemError(SmolStr::new("system failure")),
      VMError::OutOfBounds {
        ip: 1,
        index: 2,
        len: 1,
      },
      VMError::InvalidJumpTarget {
        ip: 1,
        target: 2,
        len: 1,
      },
      VMError::FeatureRestricted {
        ip: 1,
        feature: "nightly",
      },
      VMError::IoFlood { ip: 1 },
      VMError::ImportLimitReached { ip: 1 },
      VMError::UnauthorizedModule {
        ip: 1,
        module: SmolStr::new("module"),
      },
      VMError::MemoryLimitExceeded { ip: 1 },
      VMError::CallLimitExceeded { ip: 1 },
      VMError::JumpLimitExceeded { ip: 1 },
      VMError::ExcessiveNopPadding,
      VMError::InvalidMaxTicksConfig,
      VMError::TickLimitExceeded,
    ]
  }

  #[test]
  fn every_error_has_a_diagnostic_link_containing_its_code() {
    for error in all_errors() {
      assert!(
        error
          .diagnostic_link()
          .to_ascii_uppercase()
          .contains(error.error_code()),
        "diagnostic link missing {}",
        error.error_code()
      );
    }
  }

  #[test]
  fn formatted_error_contains_its_diagnostic_link() {
    set_thread_error_config(false, false, true, true);
    let error = VMError::StackOverflow { ip: 1, limit: 2 };
    let formatted = error.to_string();
    let metadata_position = formatted.find("error type:").unwrap();
    let documentation_position = formatted.find("documentation:").unwrap();
    let hint_position = formatted.find("hint:").unwrap();

    assert!(formatted.contains(&format!("documentation: {}", error.diagnostic_link())));
    assert!(formatted.contains("\x1b[36m├── \x1b[2;37mdocumentation:"));
    assert!(metadata_position < documentation_position);
    assert!(documentation_position < hint_position);
  }

  #[test]
  fn formatted_system_error_contains_its_diagnostic_link() {
    set_thread_error_config(false, false, true, true);
    let error = VMError::SystemError(SmolStr::new("system failure"));
    let formatted = error.to_string();
    let error_position = formatted.find("system failure").unwrap();
    let documentation_position = formatted.find("documentation:").unwrap();
    let hint_position = formatted.find("hint:").unwrap();

    assert!(formatted.contains(&error.diagnostic_link()));
    assert!(error_position < documentation_position);
    assert!(documentation_position < hint_position);
  }

  #[test]
  fn diagnostic_link_is_rendered_when_hints_are_disabled() {
    set_thread_error_config(false, false, false, true);
    let error = VMError::StackOverflow { ip: 1, limit: 2 };
    let formatted = error.to_string();

    assert!(formatted.contains(&error.diagnostic_link()));
    assert!(formatted.contains("documentation:"));
  }

  #[test]
  fn diagnostic_link_is_rendered_before_backtrace() {
    set_thread_error_config(true, false, true, true);
    let error = VMError::StackOverflow { ip: 1, limit: 2 };
    let formatted = error.to_string();
    let documentation_position = formatted.find("documentation:").unwrap();
    let backtrace_position = formatted.find("internal backtrace:").unwrap();

    assert!(documentation_position < backtrace_position);
  }

  #[test]
  fn diagnostic_link_can_be_disabled() {
    set_thread_error_config(false, false, true, false);
    let error = VMError::StackOverflow { ip: 1, limit: 2 };

    assert!(!error.to_string().contains("documentation:"));
  }
}
