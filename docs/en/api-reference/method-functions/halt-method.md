# Halt Method
The `halt` method is a critical control function used to immediately cease all VM execution. It terminates the current process, preventing any further bytecode instructions from being processed.

## Using TypeScript
For **TypeScript**, invoking the `halt` method forces the VM to stop instantly. This is useful for emergency shutdowns or when your logic determines that the current execution context is no longer valid.

::: code-group

<<< @/examples/methodFunctions/haltCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, the halt method provides a direct way to terminate the VM instance. Once called, the VM will stop executing, ensuring that no subsequent instructions or queued tasks are performed.

::: code-group

<<< @/examples/methodFunctions/halt_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: `Unsafe`
:::