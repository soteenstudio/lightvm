# Bench Method
After setting up your VM environment and preparing your test suites, you can utilize the `bench` method to measure and analyze the performance of your bytecode execution with high precision.

## Using TypeScript
For **TypeScript**, you can define your benchmarking logic by wrapping your execution targets with setup routines. It is recommended to use `optimizeBytecode` prior to benchmarking to ensure you are measuring the most optimized instructions.

::: code-group

<<< @/examples/methodFunctions/benchCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, you can leverage the `bench` tool through the VM tools module to configure byte sizes, initialize states, and run adaptive sample iterations. Always make sure your bytecode is optimized beforehand to get accurate and reliable performance metrics.

::: code-group

<<< @/examples/methodFunctions/bench_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: no specific capability
:::
