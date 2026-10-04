# Krates (Validate & Security)
**Krates** is the validation and security layer of **LightVM**. It checks selected bytecode structures and configured resource restrictions before and during execution.

## How Krates Works
Krates combines structural validation, security checks, and execution tick monitoring. These checks do not prove complete bytecode safety, determinism, or function reachability.

  * **Bounds Verification**: `validate_bytecode` checks that targets of `Jump`, `IfFalse`, and `Break` are below the bytecode length.
  * **Variable Validation**: `validate_vars` checks indices in `ValIdx`, `GetIdx`, `SetIdx`, `IncIdx`, and `DecIdx` against `var_count`.
  * **Function Start Bounds**: Checks that each function start address is below the bytecode length.
  * **Feature Gating**: `has_nightly_opcodes` detects `instantiate`, `import`, and `export` in serialized source; the loading interface rejects them when nightly support is disabled.
  * **Resource Quotas**: `validate_security` counts I/O, imports, object/array creation, calls, and jumps in bytecode against configured limits. The execution loop also counts these operations at runtime.
  * **Module Whitelisting**: Checks import module names against `SecurityConfig.allowed_imports` during security validation.
  * **Nop Padding**: Rejects bytecode longer than ten instructions when the `Nop` count exceeds the total instruction count divided by ten using integer division.
  * **Bypass Capability**: `unsafe_mode` bypasses `validate_security` (static quotas, import whitelisting, and `Nop` padding checks) and runtime I/O, import, allocation, call, and jump quotas. Structural validation, feature gating, stack limits, and gas checks remain active.
  * **Gas Monitoring (Tick Control)**: `GasMonitor` checks the tick limit on every execution-loop iteration. Ticks count loop iterations, not elapsed time or hardware processing cycles.
  * **Tick Validation**: Rejects a zero `max_ticks` during initialization and reports an error when the tick count reaches the limit.
