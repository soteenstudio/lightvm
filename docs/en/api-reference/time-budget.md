# Time Budget

`TimeBudget` limits how long Gazle may spend optimizing bytecode. It does not limit VM execution time.

| Value | Numeric value | Optimizer budget |
| :--- | :--- | :--- |
| `Cheap` (default) | `0` | 200 ms |
| `Normal` | `1` | 1,000 ms |
| `Expensive` | `2` | 5,000 ms |

Set the value with `setTimeBudget(TimeBudget.Cheap)` (or `set_time_budget(TimeBudget::Cheap)` on Rust) before calling `tools().optimizeBytecode(...)` (or `tools().optimize_bytecode(...)` on Rust). A larger budget lets optimization passes run longer but does not guarantee a specific optimization result.

::: info
Use `SecurityConfig.maxTicks` to limit execution work. `TimeBudget` and `maxTicks` apply to different phases.
:::
