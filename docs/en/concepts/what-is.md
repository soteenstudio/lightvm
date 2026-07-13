# What is LightVM?
**LightVM** is a high-performance, deterministic virtual machine designed to bridge the gap between human-readable logic and machine-efficient execution. Built with Rust, it prioritizes resource transparency and safety, making it an ideal runtime for embedded systems, simulation engines, and performance-critical applications.

## The Philosophy
At its core, LightVM is built on three fundamental pillars that define how it handles your code:

 * __Zero Magic (Deterministic)__: Execution is linear and fully predictable. The VM operates explicitly, meaning every instruction is executed exactly as defined, without hidden state transitions or unpredictable runtime behavior.
 * **Resource Conscious**: LightVM is engineered for a minimal memory footprint. By leveraging optimized data structures like `SmolStr` and `Ahash` for metadata management, it maintains high performance even under tight resource constraints.
 * **Explicit Security**: Security is enforced through a strict Capability system. The VM does not assume permissions; instead, every access and operation must have its rights explicitly defined by the host environment, preventing unauthorized side effects.

## Architecture: The Execution Pipeline
LightVM achieves its speed through a sophisticated pre-execution pipeline. Before a single instruction is processed by the main loop, your bytecode passes through two specialized stages designed to maximize efficiency:

### 1. Torja: The Symbol Resolver
**Torja** acts as the gateway of the VM. It transforms high-level, human-readable bytecode into a high-performance format. By mapping variable names and function identifiers to fixed-position integer indices, Torja eliminates costly runtime hash-map lookups. It also performs "Value Promotion," converting generic instructions into specialized opcodes (e.g., `push_int16` vs `push_string`), which gives the execution engine advance knowledge of data types and sizes.

### 2. Gazle: The Bytecode Optimizer
Once symbols are resolved, **Gazle** takes over to refine the bytecode. It runs a multi-pass optimization pipeline—including constant folding, dead store elimination, and jump threading—to prune unnecessary operations and simplify control flow. By the time the bytecode reaches the execution phase, it has been stripped of redundant steps, ensuring that the VM only performs work that contributes directly to the final program state.

::: tip
LightVM is designed to be lean, transparent, and fast. By separating **Resolution** (Torja) from **Optimization** (Gazle), it ensures that the core VM execution loop remains as streamlined as possible.
:::