# What is LightVM?
**LightVM** is a high-performance, deterministic virtual machine designed to bridge the gap between human-readable logic and machine-efficient execution. Built with Rust, it prioritizes resource transparency and safety, making it an ideal runtime for embedded systems, simulation engines, and performance-critical applications.

## The Philosophy
At its core, LightVM is built on three fundamental pillars that define how it handles your code:

 * __Zero Magic (Deterministic)__: Execution is linear and fully predictable. The VM operates explicitly, meaning every instruction is executed exactly as defined, without hidden state transitions or unpredictable runtime behavior.
 * **Resource Conscious**: LightVM is engineered for a minimal memory footprint. By leveraging optimized data structures like `SmolStr` and `Ahash` for metadata management, it maintains high performance even under tight resource constraints.
 * **Explicit Security**: Security is enforced through a strict Capability system. The VM does not assume permissions; instead, every access and operation must have its rights explicitly defined by the host environment, preventing unauthorized side effects.

## Architecture: The Execution Pipeline
LightVM achieves its speed through a sophisticated pre-execution pipeline. Before a single instruction is processed by the main loop, your bytecode passes through three specialized stages designed to maximize efficiency:

### 1. Torja: The Symbol Resolver
**Torja** acts as the gateway of the VM. It transforms high-level, human-readable bytecode into a high-performance format. By mapping variable names and function identifiers to fixed-position integer indices, Torja eliminates costly runtime hash-map lookups. It also performs "Value Promotion," converting generic instructions into specialized opcodes (e.g., `push_int16` vs `push_string`), which gives the execution engine advance knowledge of data types and sizes.

### 2. Gazle: The Bytecode Optimizer
**Gazle** acts as the optimization engine once symbols are resolved to refine the bytecode. It runs a multi-pass optimization pipeline—including constant folding, dead store elimination, and jump threading—to prune unnecessary operations and simplify control flow. By the time the bytecode reaches the execution phase, it has been stripped of redundant steps, ensuring that the VM only performs work that contributes directly to the final program state.

### 3. Krates: The Validation & Security Layer
**Krates** acts as the final gatekeeper that inspects bytecode before execution, ensuring the runtime remains protected against malformed instructions, unauthorized features, and memory access violations. By enforcing strict safety protocols through a comprehensive verification pipeline, Krates guarantees that only safe and deterministic bytecode reaches the execution engine. It handles critical security tasks, including bounds verification to prevent memory overflow, variable safety checks, and function integrity validation. Furthermore, Krates monitors for restricted features, enforces resource quotas via gas monitoring (ticks) to prevent infinite loops, and maintains a strict module whitelist. It also performs instruction pattern analysis to detect potentially malicious bytecode, all while offering an `unsafe_mode` configuration to bypass these checks for trusted, high-performance environments.

## Itme: The Benchmarking Utility
**Itme** is a high-precision benchmarking utility designed to measure and analyze code performance. By leveraging adaptive iteration cycles, warm-up phases, and rigorous statistical analysis using the Interquartile Range (IQR) method, Itme filters out noise and evaluates execution consistency. It automatically calculates precise operation timings, standard deviations, throughput in MiB/s, and stability percentages, ensuring reliable and reproducible performance metrics for your functions.

::: tip
LightVM is designed to be lean, transparent, and fast. By separating **Resolution** (Torja), **Optimization** (Gazle), and **Security** (Krates) into distinct pre-execution pipeline stages, and providing **Benchmarking** (Itme) as a separate performance measurement utility, LightVM ensures that the core VM execution loop remains as streamlined as possible.
:::