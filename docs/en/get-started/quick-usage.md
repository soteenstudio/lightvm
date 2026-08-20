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

::: info Related API Reference
Want to configure permissions or execution limits? Check out the [Capabilities](/api-reference/capabilities) and [Time Budget](/api-reference/time-budget) references for detailed usage.
:::