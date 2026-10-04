# What is LightVM?
**LightVM** is a high-performance, deterministic virtual machine designed to bridge the gap between human-readable logic and machine-efficient execution. Built with Rust, it prioritizes resource transparency and safety, making it an ideal runtime for embedded systems, simulation engines, and performance-critical applications.

## The Philosophy
At its core, LightVM is built on three fundamental pillars that define how it handles your code:

  * **Zero Magic (Explicit Execution)**: The execution loop dispatches bytecode instructions directly. Host imports and I/O can affect results, so determinism depends on the program and its environment.
  * **Resource Conscious**: LightVM is engineered for a minimal memory footprint. By leveraging optimized data structures like `SmolStr` and `Ahash` for metadata management, it maintains high performance even under tight resource constraints.
  * **Explicit Security**: The host configures resource quotas, allowed imports, and tick limits through `SecurityConfig`.

## Architecture: The Execution Pipeline
LightVM separates symbol resolution and validation from explicit optimization and benchmarking. Normal execution resolves symbols and validates bytecode; Gazle optimization is an optional operation before loading or execution.

### 1. Torja: The Symbol Resolver
**Torja** maps variable and function parameter names to numerical indices in one symbol table per resolution call. It converts supported name-based instructions into index-based forms without creating separate lexical scope tables.

### 2. Gazle: The Bytecode Optimizer
**Gazle** is invoked explicitly to apply passes such as constant folding, dead store elimination, and jump threading. It also specializes supported generic `push` values into type-specific instructions such as `push_int16` and `push_string`. Optimization uses a configurable time budget checked between passes, rather than a hard deadline for running passes.

### 3. Krates: The Validation & Security Layer
**Krates** checks variable indices, `Jump`/`IfFalse`/`Break` targets, and function start bounds. Security validation checks configured instruction quotas, import whitelists, and excessive `Nop` padding. Runtime quotas and gas monitoring count executed operations and execution-loop iterations. `unsafe_mode` bypasses security validation and runtime operation quotas; structural validation, feature gating, stack limits, and gas checks remain active. These checks do not guarantee complete bytecode safety or function reachability.

### 4. Itme: The Benchmarking Utility
**Itme** is a separate utility that calibrates iterations per sample, performs warm-up runs, and analyzes timing samples using IQR filtering. It reports time per operation, throughput when byte sizes are provided, and a stability metric of standard deviation / mean × 100. Lower stability values indicate less variation; results depend on the measurement environment.

::: tip
Use **Resolution** (Torja) and **Validation** (Krates) during normal execution, invoke **Optimization** (Gazle) explicitly when needed, and use **Benchmarking** (Itme) to measure performance separately.
:::
