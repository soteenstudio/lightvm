# Run Method
After initializing the VM and configuring your desired capabilities, you can load your bytecode and start the execution process.

## Using TypeScript
For **TypeScript**, you can pass raw bytecode arrays directly to the loader. It is recommended to use `optimizeBytecode` beforehand to ensure the VM runs the most efficient version of your instructions.

::: code-group

<<< @/examples/methodFunctions/runWithArray.ts{ts:line-numbers}[With Array]

:::

## Using Rust
In **Rust**, you typically work with serialized bytecode strings (or `serde_json`). Similar to the TypeScript implementation, always optimize your bytecode before loading it into the VM to maintain optimal performance.

::: code-group

<<< @/examples/methodFunctions/run_with_raw_string.rs{rust:line-numbers}[With Raw String]

<<< @/examples/methodFunctions/run_with_serde.rs{rust:line-numbers}[With Serde]

:::

::: info
**Capability Required**: `Control`
:::

::: tip
The ``.run()`` method is the final step in the execution pipeline. Ensure all necessary `capabilities` have been granted during initialization to avoid runtime security exceptions.
:::
