# Paniclog Method

The `LightVM.paniclog()` method returns the persisted paniclog records as a structured array. Each record contains bounded diagnostic context and VM state; avoid placing sensitive values in diagnostic context.

This API requires `Capability.Debug` and is unavailable in WebAssembly builds.

## Using TypeScript

The TypeScript wrapper calls the Node N-API `paniclog()` method. It returns the records array on success and follows the N-API behavior by throwing an error on failure.

::: code-group

<<< @/examples/methodFunctions/paniclogCode.ts{ts:line-numbers}[Code]

:::

## Using Rust

The native Rust `LightVM::paniclog()` method returns `Result<serde_json::Value, VMError>`. The JSON value is an array on success. On failure, the native interface prints the formatted `VMError` and returns the same error.

::: code-group

<<< @/examples/methodFunctions/paniclog_code.rs{rust:line-numbers}[Code]

:::

## Using Node N-API

The Node N-API `paniclog()` method returns the structured records array on success and throws an N-API error on authorization, storage, validation, or parsing failure.

::: info
**Capability Required**: `Capability.Debug`
:::
