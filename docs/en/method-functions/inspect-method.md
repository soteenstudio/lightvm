# Inspect Method <Badge type="warning" text="Preview" />
The `inspect` method allows you to examine the current state of the VM, providing visibility into the stack, registers, and loaded bytecode at any point during the execution cycle.

## Using TypeScript
For **TypeScript**, you can use the `inspect` method to retrieve a snapshot of the VM's internal state. This is particularly useful for debugging or monitoring the VM's behavior during runtime to ensure your logic is executing as expected.

::: code-group

<<< @/examples/methodFunctions/inspectCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, the `inspect` method provides a structured view of the VM's internals. It is designed to expose the current environment state, allowing for precise tracking of variable values and stack operations during the execution of your bytecode.

::: code-group

<<< @/examples/methodFunctions/inspect_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: `Observe`
:::