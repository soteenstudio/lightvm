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
