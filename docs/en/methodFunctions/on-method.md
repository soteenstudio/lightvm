# On Method <Badge type="warning" text="Nightly" />
The `on` method is the event-driven heart of `LightVM`, allowing you to hook into specific VM lifecycle events. By registering callbacks, you can react to internal state changes and execution milestones in real-time.

## Using TypeScript
For **TypeScript**, the `on` method allows you to attach event listeners that execute whenever a registered event is triggered. This is ideal for logging, monitoring VM state changes, or triggering external logic when specific conditions are met.

::: code-group

<<< @/examples/methodFunctions/onCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, the `on` method enables you to define event handlers that integrate directly with your system's logic. This allows for robust asynchronous notification when the VM enters a certain state, such as halting or encountering an error.

::: code-group

<<< @/examples/methodFunctions/on_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: no specific capability

**Owned Events**: `tick`, `halt`, and `panic`
:::