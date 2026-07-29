# Krates (Validate & Security)
**Krates** is the dedicated validation and security layer of **LightVM**. It acts as the final gatekeeper that inspects bytecode before execution, ensuring that the runtime environment remains protected against malformed instructions, unauthorized features, and memory access violations.

## How Krates Works
Krates enforces strict safety protocols through a comprehensive verification pipeline. By validating every instruction's integrity and compliance, it guarantees that only safe and deterministic bytecode can reach the execution engine.

 * **Bounds Verification**: Scans all jump, branch, and loop instructions to ensure the target address resides within the valid memory space, preventing out-of-bounds access.
 * **Variable Safety**: Validates that all index-based access instructions (`get_idx`, `set_idx`, etc.) reference variables within the allocated `var_count` range, stopping potential memory corruption.
 * **Function Integrity**: Cross-references all function metadata (start addresses) against the total bytecode length to ensure every call point is reachable and secure.
 * **Feature Gating**: Acts as a security guardrail by monitoring for restricted features, such as nightly opcodes, and prevents execution if the VM environment is not configured to support experimental capabilities.
