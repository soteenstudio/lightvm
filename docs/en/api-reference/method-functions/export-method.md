# Export Method <Badge type="warning" text="Nightly" />
The `export` method allows you to extract specific functions or logic registered within the VM's environment, exposing them to your host application. This effectively bridges the gap between the VM's isolated execution space and your primary code.

## Using TypeScript
For **TypeScript**, the `export` method provides a seamless way to retrieve and invoke functions defined within the VM. By exporting a function, you can treat VM-side logic as native code, simplifying interaction and data processing between the two environments.

::: code-group

<<< @/examples/methodFunctions/exportCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, the `export` method is used to map VM-side functions to Rust-callable interfaces. This mechanism allows you to leverage VM execution results directly within your main Rust application, enabling high-performance integration and modular architecture.

::: code-group

<<< @/examples/methodFunctions/export_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: `Control`
:::