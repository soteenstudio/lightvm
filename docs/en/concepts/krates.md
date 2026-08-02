# Krates (Validate & Security)
**Krates** is the dedicated validation and security layer of **LightVM**. It acts as the final gatekeeper that inspects bytecode before execution, ensuring that the runtime environment remains protected against malformed instructions, unauthorized features, and memory access violations.

## How Krates Works
Krates enforces strict safety protocols through a comprehensive verification pipeline. By validating every instruction's integrity and compliance, it guarantees that only safe and deterministic bytecode can reach the execution engine.

 * **Bounds Verification**: Scans all jump, branch, and loop instructions to ensure the target address resides within the valid memory space, preventing out-of-bounds access.
 * **Variable Safety**: Validates that all index-based access instructions (`get_idx`, `set_idx`, etc.) reference variables within the allocated `var_count` range, stopping potential memory corruption.
 * **Function Integrity**: Cross-references all function metadata (start addresses) against the total bytecode length to ensure every call point is reachable and secure.
 * **Feature Gating**: Acts as a security guardrail by monitoring for restricted features, such as nightly opcodes, and prevents execution if the VM environment is not configured to support experimental capabilities.
 * **Resource Quota Enforcement**: Implements strict limits on system operations to prevent resource exhaustion, including caps on I/O operations, imports, memory allocations, and control flow jumps.
 * **Module Whitelisting**: Ensures that only pre-approved modules defined in the `SecurityConfig` can be imported, mitigating risks from unauthorized external code.
 * **Instruction Pattern Analysis**: Detects malicious bytecode patterns, such as excessive `Nop` padding, which could be used to bypass analysis or bloat execution time.
 * **Bypass Capability**: Supports an `unsafe_mode` configuration that allows for the explicit disabling of security checks, intended for trusted, high-performance environments where overhead must be minimized.
 * **Gas Monitoring (Tick Control)**: Utilizes the `GasMonitor` system to track execution time or complexity via "ticks." It enforces a strict upper bound on processing cycles to prevent infinite loops or runaway execution, ensuring the VM remains responsive and deterministic.
 * **Tick Validation**: Validates `SecurityConfig` during initialization to ensure non-zero tick limits, preventing invalid or insecure configuration states before the runtime begins.
