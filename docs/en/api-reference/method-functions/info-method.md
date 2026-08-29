# Info Method
After initializing the VM, you can retrieve metadata, system versions, and update statuses without needing to run any execution cycles.

## Using TypeScript
For **TypeScript**, you can call the info method directly on the VM instance to fetch current version details and check for pending updates asynchronously.

::: code-group

<<< @/examples/methodFunctions/infoCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, you can query the initialized VM instance to inspect current runtime versions and update availability by calling the `.info()` method, which returns an `InfoVM` struct.

::: code-group

<<< @/examples/methodFunctions/info_code.rs{rust:line-numbers}[Code]

:::
