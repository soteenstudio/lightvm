# Time Budget

`TimeBudget` sets how much time Gazle can spend optimizing bytecode. It does not limit VM execution time.

| Value | Numeric value | Optimizer budget |
| :--- | :--- | :--- |
| `Cheap` (default) | `0` | 200 ms |
| `Normal` | `1` | 1,000 ms |
| `Expensive` | `2` | 5,000 ms |

Set the budget before starting optimization. Use `setTimeBudget(TimeBudget.Cheap)` before `tools().optimizeBytecode(...)`, or `set_time_budget(TimeBudget::Cheap)` before `tools().optimize_bytecode(...)` in Rust.

A larger budget allows optimization to continue for longer. It does not guarantee a particular result.

::: info
`SecurityConfig.maxTicks` limits VM execution work independently of `TimeBudget`.
:::
