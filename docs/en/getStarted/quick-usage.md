# Quick Usage
LightVM initialization is **very flexible** and allows you to configure the **capabilities** and **debugging features** according to your application needs.

## Using TypeScript
For **TypeScript-based** projects, you can configure a VM instance with an intuitive builder pattern before accessing the main `tools` interface.

::: code-group

<<< @/examples/getStarted/builderPattern.ts{ts:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/objectPattern.ts{ts:line-numbers}[Object Pattern]

:::

## Using Rust
For **Rust** users, configuration is done through `VmConfig`. You can declaratively set VM capabilities before executing bytecode.

::: code-group

<<< @/examples/getStarted/builder_pattern.rs{rust:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/object_pattern.rs{rust:line-numbers}[Object Pattern]

:::

::: tip
Get the tools interface. Store this as a constant to reuse it for all upcoming tasks.
:::

## Capabilities
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