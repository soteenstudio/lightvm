# Gazle (Optimizer)
**Gazle** is a built-in optimizer that comes with **LightVM** to perform optimizations before your bytecode is executed through a multi-pass optimization method to ensure maximum performance and minimal footprint.

## How Gazle Works
Gazle employs a specialized transformation pipeline to refine your bytecode. It processes the instruction stream through multiple passes, systematically identifying and eliminating inefficiencies before the VM runtime even begins.

 * **Constant Folding**: Pre-calculates math and logic operations (e.g., `add`, `sub`, `xor`, `concat`) if the values are known at compile-time.
 * **Conversion & Metadata Folding**: Pre-evaluates type casting (e.g., `to_integer`, `to_string`) and metadata checks like TypeOf to eliminate redundant runtime work.
 * **Strength Reduction**: Replaces "heavy" operations with lighter ones, such as converting multiplication by powers of two into bitwise `shl` (Shift Left).
 * **Dead Store Elimination**: Analyzes variable usage and automatically removes `push`, `set`, or `inc` operations that don't contribute to the final program state.
 * **Dead Loop Elimination**: Identifies and prunes "pure" loops that have no side effects (no I/O, calls, or returns), preventing unnecessary CPU cycles.
 * **Redundant Load Elimination**: Detects consecutive attempts to load identical values or variables onto the stack and replaces redundant operations with a high-performance `dup` instruction to minimize memory access overhead.
 * **Jump Optimization**: Detects and removes redundant Jump instructions that point to the very next line of code.
 * **Jump Threading**: Optimizes control flow by collapsing chains of redirection, where a jump leads directly to another jump, ensuring the instruction pointer bypasses intermediate hops to reach the final destination immediately.

::: info
You can find how to use Gazle on the [Optimize Bytecode Method](../api-references/method-functions/tools-method/optimize-bytecode-method) page.
:::