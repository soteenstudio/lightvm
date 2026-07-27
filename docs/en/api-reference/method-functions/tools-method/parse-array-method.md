# Parse Array Method
The `parseLTCArray` method is designed to transform human-readable LightVM assembly strings into a structured, machine-parsable JSON array. While the standard `parse` method handles general translation, `parseLTCArray` is specifically tailored for scenarios where you need direct access to the serialized bytecode structure, making it ideal for debugging, static analysis, or creating external build tools.

## Using TypeScript
In **TypeScript**, the `parseLTCArray` method converts your assembly logic into a standard JSON array format. This is super handy if you're building frontend tools, dashboards, or simply need to inspect your opcode sequences in a familiar JavaScript object format before runtime.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parseLTCArrayCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
For **Rust** developers, this method provides a seamless way to convert raw instruction strings into the serialized bytecode format. It’s perfect for creating CLI utilities or custom build pipelines where you need to generate, validate, or pre-process instruction arrays programmatically.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parse_array_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: no specific capability
:::
