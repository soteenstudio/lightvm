# Gazle (Optimizer)
**Gazle** is a built-in optimizer for **LightVM**, invoked explicitly before loading or executing bytecode. Its adaptive multi-pass method uses configurable budgets: Cheap = 200 ms, Normal = 1,000 ms, and Expensive = 5,000 ms.

## How Gazle Works
Gazle uses a **Time-Budgeted Optimization** mechanism and orders passes by success-based weights (`pass_weights`). Budget checks occur between passes; a running pass can exceed the selected budget. Optimization is not a mandatory stage of normal execution.

  * **Specialized Instructions**: Converts supported generic `push` values into type-specific instructions such as `push_int16`, `push_string`, and `push_bool`.
  * **Constant Folding**: Pre-calculates math and logic operations (e.g., `add`, `sub`, `xor`, `concat`) if the values are known at compile-time.
  * **Conversion & Metadata Folding**: Pre-evaluates type casting (e.g., `to_integer`, `to_string`) and metadata checks like `type_of` to eliminate redundant runtime work.
  * **Strength Reduction**: Replaces "heavy" operations with lighter ones, such as converting multiplication by powers of two into bitwise `shl` (Shift Left).
  * **Dead Store Elimination**: Uses variable-read information and a backward stack-demand scan to replace selected unused stores, increments/decrements, and value-producing instructions with `Nop`.
  * **Dead Loop Elimination**: The current pass selects backward `Jump` and `IfFalse` instructions as loop candidates. Its purity check includes and rejects those terminating control-flow instructions, so it does not remove the loops it selects.
  * **Redundant Load Elimination**: Replaces supported consecutive identical loads with `dup`, including repeated `get`/`get_idx` and generic `push` values.
  * **Jump Optimization**: Detects and removes redundant Jump instructions that point to the very next line of code.
  * **Jump Threading**: Optimizes control flow by collapsing chains of redirection, where a jump leads directly to another jump, ensuring the instruction pointer bypasses intermediate hops to reach the final destination immediately.
  * **Constant Propagation**: Optimizes bytecode by tracking variable assignments and replacing `get` operations with direct `push` instructions when values are known constants. It checks usage frequency and avoids inlining heavy objects like arrays or objects, clearing tracked constants at `Jump` and `IfFalse` instructions.

::: info
You can find how to use Gazle on the [Optimize Bytecode Method](../api-reference/method-functions/tools-method/optimize-bytecode-method) page.
:::
