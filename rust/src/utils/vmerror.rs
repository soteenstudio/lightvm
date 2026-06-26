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
use std::sync::Mutex;
use std::sync::OnceLock;
pub struct VMErrorContainer {
  explain: bool,
  hint: bool,
}
impl Default for VMErrorContainer {
  fn default() -> Self {
    Self::new()
  }
}
impl VMErrorContainer {
  pub fn new() -> Self {
    Self {
      explain: false,
      hint: true,
    }
  }
  pub fn get_value(&self) -> VMErrorContainer {
    VMErrorContainer {
      explain: self.explain,
      hint: self.hint,
    }
  }
  pub(crate) fn set_value(&mut self, explain: bool, hint: bool) {
    self.explain = explain;
    self.hint = hint;
  }
}
static EXPLAIN_MODE: OnceLock<Mutex<VMErrorContainer>> = OnceLock::new();
pub fn get_error_config() -> &'static Mutex<VMErrorContainer> {
  EXPLAIN_MODE.get_or_init(|| Mutex::new(VMErrorContainer::new()))
}
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
impl fmt::Display for VMError {
  #[cold]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let red = "\x1b[31;1m";
    let _yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let dark_gray = "\x1b[2;37m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";
    let config = get_error_config().lock().unwrap().get_value();
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
      VMError::SystemError(_) => "SystemError",
    };
    let ip = match self {
      VMError::SystemError(_) => 0,
      VMError::StackOverflow { ip, .. }
      | VMError::StackUnderflow { ip, .. }
      | VMError::InvalidOpcode { ip, .. }
      | VMError::TypeMismatch { ip, .. }
      | VMError::OutOfBounds { ip, .. }
      | VMError::InvalidJumpTarget { ip, .. }
      | VMError::FeatureRestricted { ip, .. } => *ip,
    };
    write!(f, "{bold}{red}Error[{}]{reset}: ", self.error_code())?;
    match self {
      VMError::StackOverflow { limit, .. } => write!(f, "Stack limit reached (limit: {}).", limit),
      VMError::StackUnderflow { opcode, .. } => write!(
        f,
        "Attempted to pop from an empty stack during {bold}'{}' {reset}instruction.",
        opcode,
      ),
      VMError::InvalidOpcode { code, .. } => {
        write!(
          f,
          "Illegal instruction {bold}'{}' {reset}encountered.",
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
          "The feature/opcode {bold}'{}' {reset}is restricted.",
          feature
        )
      }
      VMError::SystemError(s) => write!(f, "{}", s),
    }?;
    if !matches!(self, VMError::SystemError(_)) {
      if is_hint {
        write!(
          f,
          "\n {reset}{cyan}│   {dark_gray}at instruction pointer: {ip}{reset}"
        )?;
        write!(f, "\n {reset}{cyan}│   {dark_gray}error type: {}", err_type)?;
      } else {
        write!(f, "\n     {dark_gray}at instruction pointer: {ip}{reset}")?;
        write!(f, "\n     {dark_gray}error type: {}", err_type)?;
      }
    }
    match self {
      VMError::StackOverflow { .. } => {
        if is_hint {
          if is_explain {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The execution encountered a stack overflow, likely triggered by either an infinitely recursive function call that never terminates or an InitStack instruction that failed to reserve enough space for the required stack depth, resulting in the runtime exceeding the allocated memory boundaries for the current call frame.{reset}\n\n"
            )
          } else {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}Potential infinite recursion or unoptimized InitStack.{reset}\n\n"
            )
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::StackUnderflow { .. } => {
        if is_hint {
          if is_explain {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The stack is currently unbalanced because more elements were popped than pushed; this indicates that your bytecode logic is attempting to access data that was never placed onto the stack, or the previous instructions failed to maintain the required stack integrity.{reset}\n\n"
            )
          } else {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}Stack state is inconsistent; check your push/pop balance.{reset}\n\n"
            )
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::InvalidOpcode { .. } => {
        if is_hint {
          if is_explain {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The stack is currently unbalanced because more elements were popped than pushed; this indicates that your bytecode logic is attempting to access data that was never placed onto the stack, or the previous instructions failed to maintain the required stack integrity.{reset}\n\n"
            )
          } else {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}Bytecode may be corrupted or version mismatch.{reset}\n\n"
            )
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::TypeMismatch { .. } => {
        if is_hint {
          if is_explain {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The data structure or value provided to this specific instruction does not adhere to the required type definition or parameter signature, which forces the runtime to halt because it cannot safely proceed with an operation expecting a different format.{reset}\n\n"
            )
          } else {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}Ensure the data passed to this instruction matches the expected signature.{reset}\n\n"
            )
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::OutOfBounds { len, .. } => {
        if is_hint {
          if *len == 0 {
            if is_explain {
              write!(
                f,
                "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The collection currently being accessed contains no elements, making any attempt to retrieve an index operationally invalid because there is no allocated data at any position to be retrieved.{reset}\n\n"
              )
            } else {
              write!(
                f,
                "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}Collection is empty; no index is valid.{reset}\n\n"
              )
            }
          } else {
            if is_explain {
              write!(
                f,
                "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The requested index falls outside the valid memory boundaries of the collection; you must ensure your index calculation is strictly constrained between zero and the collection's length minus one, as off-by-one errors are the primary cause of this boundary violation.{reset}\n\n"
              )
            } else {
              write!(
                f,
                "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}Verify your index calculation; ensure it is within 0 to {}. Off-by-one errors are common here.{reset}\n\n",
                len.saturating_sub(1)
              )
            }
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::InvalidJumpTarget { .. } => {
        if is_hint {
          if is_explain {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The jump operation attempted to redirect the instruction pointer to a memory address that does not exist within the current bytecode bounds, indicating either a critical corruption of the jump offset or a logical error in the flow control mapping.{reset}\n\n"
            )
          } else {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}The jump target is out of range. Check for corrupted bytecode or logic errors in your jump instructions.{reset}\n\n"
            )
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::FeatureRestricted { .. } => {
        if is_hint {
          if is_explain {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint (explained):\n    {dark_gray}The attempt to execute this instruction was blocked because it is classified as an experimental or restricted feature; you must explicitly enable 'nightly mode' within your VmConfig to authorize the runtime to process this opcode.{reset}\n\n"
            )
          } else {
            write!(
              f,
              "\n {reset}{cyan}│\n {cyan}└─ {cyan}hint: {dark_gray}You need to enable nightly mode in VmConfig to use it.{reset}\n\n"
            )
          }
        } else {
          write!(f, "{reset}\n\n")
        }
      }
      VMError::SystemError(_) => Ok(()),
    }
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
      VMError::FeatureRestricted { .. } => "LVM007",
      VMError::SystemError(_) => "LVM500",
    }
  }
}
