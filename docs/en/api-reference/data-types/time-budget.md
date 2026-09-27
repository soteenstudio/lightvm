# Time Budget

`TimeBudget` sets how much time Gazle can spend optimizing bytecode. It does not limit VM execution time.

| Value | Numeric value | Optimizer budget |
| :--- | :--- | :--- |
| `Cheap` (default) | `0` | 200 ms |
| `Normal` | `1` | 1,000 ms |
| `Expensive` | `2` | 5,000 ms |
