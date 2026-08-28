# Time Budget
Execution limits in LightVM are managed through preset time budgets to prevent infinite loops and runaway scripts. Use the following table to understand the available execution limits and duration levels:

| Budget Level | Max Ticks | Description |
| :--- | :--- | :--- |
| `Cheap` **(Default)** | 200 ticks | Optimized for fast, lightweight script executions and quick validations. |
| `Normal` | 1000 ticks | Standard limit suitable for general-purpose applications. |
| `Expensive` | 5000 ticks | Extended execution window for heavy computations or complex logic. |

::: info
**Time Budget** is enforced during execution by the GasMonitor, which checks the tick count on every VM instruction. Both `set_time_budget` and `set_max_ticks` set the same underlying `max_ticks` limit, so whichever method is called most recently determines the effective execution limit.
:::

::: warning Performance Notice
Choose the appropriate budget level carefully. Setting an unnecessarily high budget for untrusted scripts may risk resource exhaustion or hanging execution threads.
:::