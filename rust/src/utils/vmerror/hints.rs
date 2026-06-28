/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::vmerror::error::{Hint, VMError};
use std::borrow::Cow;
pub fn get_hint(err: &VMError) -> Option<Hint> {
  match err {
    VMError::StackOverflow { .. } => Some(Hint {
      short: Cow::Borrowed("Potential infinite recursion or unoptimized InitStack."),
      long: Cow::Borrowed(
        "The execution encountered a stack overflow, likely triggered by either an infinitely recursive function call that never terminates or an InitStack instruction that failed to reserve enough space for the required stack depth, resulting in the runtime exceeding the allocated memory boundaries for the current call frame.",
      ),
    }),
    VMError::StackUnderflow { .. } => Some(Hint {
      short: Cow::Borrowed("Stack state is inconsistent; check your push/pop balance."),
      long: Cow::Borrowed(
        "The stack is currently unbalanced because more elements were popped than pushed; this indicates that your bytecode logic is attempting to access data that was never placed onto the stack, or the previous instructions failed to maintain the required stack integrity.",
      ),
    }),
    VMError::InvalidOpcode { .. } => Some(Hint {
      short: Cow::Borrowed("Bytecode may be corrupted or version mismatch."),
      long: Cow::Borrowed(
        "The stack is currently unbalanced because more elements were popped than pushed; this indicates that your bytecode logic is attempting to access data that was never placed onto the stack, or the previous instructions failed to maintain the required stack integrity.",
      ),
    }),
    VMError::TypeMismatch { .. } => Some(Hint {
      short: Cow::Borrowed(
        "Ensure the data passed to this instruction matches the expected signature.",
      ),
      long: Cow::Borrowed(
        "The data structure or value provided to this specific instruction does not adhere to the required type definition or parameter signature, which forces the runtime to halt because it cannot safely proceed with an operation expecting a different format.",
      ),
    }),
    VMError::OutOfBounds { len, .. } => {
      let short = if *len == 0 {
        Cow::Borrowed("Collection is empty; no index is valid.")
      } else {
        Cow::Owned(format!(
          "Verify your index calculation; ensure it is within 0 to {}. Off-by-one errors are common here.",
          len.saturating_sub(1)
        ))
      };
      let long = if *len == 0 {
        Cow::Borrowed(
          "The collection currently being accessed contains no elements, making any attempt to retrieve an index operationally invalid because there is no allocated data at any position to be retrieved.",
        )
      } else {
        Cow::Borrowed(
          "The requested index falls outside the valid memory boundaries of the collection; you must ensure your index calculation is strictly constrained between zero and the collection's length minus one, as off-by-one errors are the primary cause of this boundary violation.",
        )
      };
      Some(Hint { short, long })
    }
    VMError::InvalidJumpTarget { .. } => Some(Hint {
      short: Cow::Borrowed(
        "The jump target is out of range. Check for corrupted bytecode or logic errors in your jump instructions.",
      ),
      long: Cow::Borrowed(
        "The jump operation attempted to redirect the instruction pointer to a memory address that does not exist within the current bytecode bounds, indicating either a critical corruption of the jump offset or a logical error in the flow control mapping.",
      ),
    }),
    VMError::FeatureRestricted { .. } => Some(Hint {
      short: Cow::Borrowed("You need to enable nightly mode in VmConfig to use it."),
      long: Cow::Borrowed(
        "The attempt to execute this instruction was blocked because it is classified as an experimental or restricted feature; you must explicitly enable 'nightly mode' within your VmConfig to authorize the runtime to process this opcode.",
      ),
    }),
    VMError::SystemError(_) => Some(Hint {
      short: Cow::Borrowed("System-level operation failed."),
      long: Cow::Borrowed(
        "The runtime encountered an error interacting with the host system or OS. Check your environment permissions, available memory, or system logs for more details.",
      ),
    }),
  }
}
