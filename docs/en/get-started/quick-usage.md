# Quick Usage

Create and configure a `LightVM` instance. Complete [Installation](/get-started/installation) first.

## Using TypeScript
For TypeScript projects, configure the constructor or use the chainable configuration methods.

::: code-group

<<< @/examples/getStarted/builderPattern.ts{ts:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/objectPattern.ts{ts:line-numbers}[Object Pattern]

:::

## Using Rust
For Rust projects, configure `VmConfig` before creating the VM.

::: code-group

<<< @/examples/getStarted/builder_pattern.rs{rust:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/object_pattern.rs{rust:line-numbers}[Object Pattern]

:::

## Expected result

You have a configured VM instance ready to load bytecode. Continue with the [Run Method](/api-reference/method-functions/run-method), or review [Capabilities](/api-reference/capabilities) before granting access.

::: info Related API Reference
`TimeBudget` limits bytecode optimization time. Use `SecurityConfig.maxTicks` to limit execution work.
:::
