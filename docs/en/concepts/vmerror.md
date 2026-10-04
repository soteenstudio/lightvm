# VMError
**VMError** is a built-in structured error-reporting module that comes with **LightVM** to describe runtime failures, validation errors, and host-system problems with error codes and contextual diagnostics. The Rust `VMError` enum preserves details about each failure, while its formatter produces color-coded messages with configurable debugging information.

## How VMError Works
VMError uses a **Structured Error Enum (`VMError`)** to represent failures and a **Diagnostic Formatter (`Display`)** to turn them into readable reports. The formatter reads thread-local configuration when available, otherwise uses the global configuration, and combines the error message with metadata, documentation links, optional internal backtraces, and hints. Formatting an error does not itself terminate the process; the caller determines how the failure is handled.

  * **Structured Rust Error Variants**: Represents stack overflow and underflow, invalid opcodes, type mismatches, out-of-bounds access, invalid jump targets, restricted features, resource limits, unauthorized modules, invalid values, and system errors. Variant fields preserve relevant details such as stack limits, expected and actual types, collection lengths, and module names.
  * **Instruction Pointer Context**: Includes the stored `ip` and error type in reports for variants carrying instruction context. `ExcessiveNopPadding`, `InvalidMaxTicksConfig`, and `TickLimitExceeded` have no stored `ip`, so the formatter displays `0` as a placeholder. Rust `SystemError` reports omit instruction-pointer and error-type metadata.
  * **Diagnostic Error Codes**: Maps runtime and validation variants to `LVM001`–`LVM017` and `SystemError` to `LVM500` through `error_code()`, providing consistent identifiers for troubleshooting.
  * **Configurable Hints & Explanations**: Uses `VMErrorContainer` settings (`hint`, `explain`, `diagnostic_links`, and `backtrace`) to control diagnostic output. Hints and documentation links are enabled by default, while explanations and backtraces are disabled. When hints are enabled, `explain` selects the longer explanation instead of the short hint.
  * **Error Documentation Links**: Uses `diagnostic_link()` to generate a documentation URL for each error code. The `diagnostic_links` setting controls whether the formatter includes this URL, independently of hints.
  * **Optional Internal Backtraces**: When `backtrace` is enabled, formats a previously captured thread-local stack snapshot with up to five matching LightVM symbols, excluding the backtrace helpers. If no snapshot or matching symbols are available, the report includes an unavailable-backtrace message.
  * **Dying Color Constants**: Reuses [Dying](./dying)'s `RED` and `BOLD` for the error heading, `CYAN` for diagnostic branches, `DARK_GRAY` for supporting details, and `RESET` to reset styling. These constants provide visual formatting without making VMError a process-termination mechanism.
  * **TypeScript System Error Class**: TypeScript's `VMSystemError`, exported as `VMError`, extends `Error` and represents only `LVM500` system errors, rather than all Rust error variants. It stores `detail`, `hintDetails`, and an `ip` of `0`; its `print(explain, hint)` method writes a formatted report with `console.error` and captures a JavaScript stack trace when supported.

::: info
You can find diagnostic references on the [LVM001](../api-reference/error-codes/lvm001-code) through [LVM017](../api-reference/error-codes/lvm017-code) and [LVM500](../api-reference/error-codes/lvm500-code) pages.
:::
