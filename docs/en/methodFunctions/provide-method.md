# Provide Method
After defining your functions or logic, the `provide` method is used to register them into the VM's context, making them accessible during execution.

## Using TypeScript
For **TypeScript**, you can provide functions or data structures directly into the VM context. Ensure that the types provided align with the expected interface of the `LightVM` instance.

::: code-group

<<< @/examples/methodFunctions/provideWithArray.ts{ts:line-numbers}[With Array]

:::

## Using Rust
In **Rust**, you can provide your logic using raw string representations or by utilizing `serde_json` for more complex data structures. This allows the VM to seamlessly integrate Rust-defined logic into its operational scope.

::: code-group

<<< @/examples/methodFunctions/provide_with_raw_string.rs{rust:line-numbers}[With Raw String]

<<< @/examples/methodFunctions/provide_with_serde.rs{rust:line-numbers}[With Serde]

:::

::: info
**Capability Required**: no specific capability
:::