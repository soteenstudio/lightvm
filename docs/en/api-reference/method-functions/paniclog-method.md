# Paniclog Method
The `paniclog` method returns persisted panic records as a structured array. Each record contains bounded diagnostic context and VM state. Avoid placing sensitive values in the diagnostic context.

## Using TypeScript
For **TypeScript**, call `.paniclog()` on a VM configured with the `Debug` capability. The method returns the records array and throws an error if the records cannot be retrieved or parsed.

::: code-group

<<< @/examples/methodFunctions/paniclogCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, call `.paniclog()` on a VM configured with the `Debug` capability. The method returns `Result<serde_json::Value, VMError>`, where the JSON value is an array of panic records.

::: code-group

<<< @/examples/methodFunctions/paniclog_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: `Debug`
:::

::: warning
The `.paniclog()` method is unavailable in WebAssembly builds.
:::
