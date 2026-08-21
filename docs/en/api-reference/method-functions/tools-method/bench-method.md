# Bench Method
After setting up your VM environment and preparing your test suites, you can utilize the `bench` method to measure and analyze the performance of your bytecode execution with high precision.

## Using TypeScript
For **TypeScript**, you can define your benchmarking logic by wrapping your execution targets with setup routines. It is recommended to use `optimizeBytecode` prior to benchmarking to ensure you are measuring the most optimized instructions.

The benchmark configuration accepts the following parameters:
- `targetTime`: The target duration for benchmark execution in **milliseconds** (must be greater than zero)
- `bytes`: The number of bytes processed **per iteration** (optional, used for throughput calculation)
- `samples`: The number of sample iterations to collect (must be greater than zero)

::: code-group

<<< @/examples/methodFunctions/toolsMethod/benchCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, you can leverage the `bench` tool through the VM tools module to configure byte sizes, initialize states, and run adaptive sample iterations. Always make sure your bytecode is optimized beforehand to get accurate and reliable performance metrics.

The benchmark configuration accepts the following parameters:
- `target_time(Duration)`: The target duration for benchmark execution (must be greater than zero)
- `bytes`: The number of bytes processed **per iteration** (optional, used for throughput calculation)
- `samples`: The number of sample iterations to collect (must be greater than zero)

::: code-group

<<< @/examples/methodFunctions/toolsMethod/bench_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: `Debug`
:::
