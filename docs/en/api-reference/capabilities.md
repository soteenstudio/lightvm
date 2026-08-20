# Capabilities
The security and behavior of `LightVM` are managed through a robust capability system. Use the following table to understand which permissions are required for your specific use case:

| Capability | Level | Description |
|------------|-------|-------------|
| `Control` | Low | Grants permission to start/stop execution and export functions. |
| `Observe` | Medium | Allows the host to inspect internal states, variable stacks, and metrics. |
| `Debug` | High | Opens access to verbose internal logs and hidden states for troubleshooting. |
| `Unsafe` | Critical | Removes safety guards, allowing manual halts and raw memory/process access. |

::: warning Security Notice
Always adhere to the **Principle of Least Privilege**. Only enable the specific capabilities required for your application to ensure a secure and predictable execution environment.
:::