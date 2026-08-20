# Time Budget
Execution limits in LightVM are managed through preset time budgets to prevent infinite loops and runaway scripts. Use the following table to understand the available execution limits and duration levels:

| Budget Level | Duration | Description |
| :--- | :--- | :--- |
| `Cheap` **(Default)** | ~200ms | Optimized for fast, lightweight script executions and quick validations. |
| `Normal` | ~1000ms | Standard limit suitable for general-purpose applications. |
| `Expensive` | ~5000ms | Extended execution window for heavy computations or complex logic. |

::: warning Performance Notice
Choose the appropriate budget level carefully. Setting an unnecessarily high budget for untrusted scripts may risk resource exhaustion or hanging execution threads.
:::