/*
 * Copyright 2025-2026 SoTeen Studio
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
        "The runtime encountered an opcode that is not recognized for the current bytecode format. This usually means the bytecode is corrupted, was generated for a different VM version, or the instruction stream became misaligned.",
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
    VMError::IoFlood { .. } => Some(Hint {
      short: Cow::Borrowed("Reduce I/O operations or increase max_io in SecurityConfig."),
      long: Cow::Borrowed(
        "The bytecode exceeded the maximum number of permitted I/O operations (print, println, stdout, stdin, etc.) defined in the SecurityConfig. To resolve this, either refactor the code to reduce the number of I/O calls, or increase the max_io limit in your security configuration if the usage is legitimate.",
      ),
    }),
    VMError::ImportLimitReached { .. } => Some(Hint {
      short: Cow::Borrowed("Reduce module imports or increase max_import in SecurityConfig."),
      long: Cow::Borrowed(
        "The bytecode contains more module import statements than the maximum allowed by the SecurityConfig. You can fix this by consolidating imports, removing unused modules, or raising the max_import threshold if additional imports are necessary for your use case.",
      ),
    }),
    VMError::UnauthorizedModule { .. } => Some(Hint {
      short: Cow::Borrowed("Add the module to allowed_imports in SecurityConfig or remove the import."),
      long: Cow::Borrowed(
        "The bytecode attempted to import a module that is not included in the SecurityConfig's allowed_imports whitelist. To proceed, either add the required module name to the whitelist, or remove the import if it is not essential. This restriction prevents execution of untrusted or unauthorized external code.",
      ),
    }),
    VMError::MemoryLimitExceeded { .. } => Some(Hint {
      short: Cow::Borrowed("Reduce object/array allocations or increase max_alloc in SecurityConfig."),
      long: Cow::Borrowed(
        "The bytecode exceeded the maximum permitted memory allocations (MakeObj, MakeArray) as defined in the SecurityConfig. To resolve this, optimize your data structures to use fewer allocations, reuse existing objects where possible, or increase the max_alloc limit if the memory usage is justified.",
      ),
    }),
    VMError::CallLimitExceeded { .. } => Some(Hint {
      short: Cow::Borrowed("Reduce function calls or increase max_call in SecurityConfig."),
      long: Cow::Borrowed(
        "The bytecode contains more function call instructions than the maximum allowed by the SecurityConfig. This limit prevents excessively complex or potentially malicious call chains. You can address this by refactoring to reduce recursion depth or call frequency, or by increasing the max_call threshold if the call pattern is legitimate.",
      ),
    }),
    VMError::JumpLimitExceeded { .. } => Some(Hint {
      short: Cow::Borrowed("Reduce control flow complexity or increase max_jump in SecurityConfig."),
      long: Cow::Borrowed(
        "The bytecode exceeded the maximum number of control flow jump instructions (Jump, IfFalse, Break) permitted by the SecurityConfig. This typically indicates overly complex branching or loop structures. Simplify your control flow logic, reduce nested loops, or increase the max_jump limit if the complexity is unavoidable.",
      ),
    }),
    VMError::ExcessiveNopPadding => Some(Hint {
      short: Cow::Borrowed("Remove unnecessary Nop instructions from the bytecode."),
      long: Cow::Borrowed(
        "The bytecode contains an excessive proportion of Nop (no-operation) instructions, which may indicate an attempt to obfuscate code, bypass analysis, or artificially inflate the bytecode size. Review and regenerate the bytecode to eliminate unnecessary padding. If the Nops are intentional, they exceed the 10% threshold relative to total instructions.",
      ),
    }),
    VMError::InvalidMaxTicksConfig => Some(Hint {
      short: Cow::Borrowed("Set max_ticks to a value greater than zero in SecurityConfig."),
      long: Cow::Borrowed(
        "The SecurityConfig was initialized with a max_ticks value of zero, which is invalid and would allow unbounded execution. You must configure max_ticks to a positive integer to enforce execution limits and prevent infinite loops or runaway processes. Update your configuration before initializing the VM.",
      ),
    }),
    VMError::TickLimitExceeded => Some(Hint {
      short: Cow::Borrowed("Optimize execution or increase max_ticks in SecurityConfig."),
      long: Cow::Borrowed(
        "The execution exceeded the maximum number of ticks (complexity/time units) allowed by the SecurityConfig. This limit prevents infinite loops and ensures the VM remains responsive. To fix this, optimize your code to reduce computational complexity, or increase the max_ticks threshold if the workload legitimately requires more processing cycles.",
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
