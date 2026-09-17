# Clear Paniclog Method

The `LightVM.clearPaniclog()` method removes all persisted paniclog records. This API requires `Capability.Debug` and is unavailable in WebAssembly builds.

## Using TypeScript

The TypeScript wrapper calls the Node N-API `clearPaniclog()` method. It returns normally after the records are removed and follows the N-API behavior by throwing an error on failure.

::: code-group

<<< @/examples/methodFunctions/clearPaniclogCode.ts{ts:line-numbers}[Code]

:::

## Using Rust

The equivalent native Rust method is `LightVM::clear_paniclog()`, which returns `Result<(), VMError>`. On failure, the native interface prints the formatted `VMError` and returns the same error.

::: code-group

<<< @/examples/methodFunctions/clear_paniclog_code.rs{rust:line-numbers}[Code]

:::

## Using Node N-API

The Node N-API `clearPaniclog()` method removes the persisted records on success and throws an N-API error on authorization or storage failure.

::: info
**Capability Required**: `Capability.Debug`
:::
