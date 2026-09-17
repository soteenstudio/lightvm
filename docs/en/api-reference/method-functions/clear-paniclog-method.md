# Clear Paniclog Method
The `clearPaniclog` method removes all persisted panic records.

## Using TypeScript
For **TypeScript**, call `.clearPaniclog()` on a VM configured with the `Debug` capability. The method returns after the records are removed and throws an error if the operation fails.

::: code-group

<<< @/examples/methodFunctions/clearPaniclogCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, call `.clear_paniclog()` on a VM configured with the `Debug` capability. The method returns `Result<(), VMError>`.

::: code-group

<<< @/examples/methodFunctions/clear_paniclog_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: `Debug`
:::

::: warning
The `.clearPaniclog()` and `.clear_paniclog()` methods are unavailable in WebAssembly builds.
:::
