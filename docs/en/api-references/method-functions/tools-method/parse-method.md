# Parse Method
The `parse` method is the inverse of `stringify`. It allows you to convert human-readable LightVM instruction strings back into executable bytecode structures. This is essential when you want to write your logic in a readable format and have the VM handle the compilation or translation process.

## Using TypeScript
For **TypeScript**, you can pass your raw instruction strings directly to the `parse` method. This will translate your code into the structured bytecode format required by the VM.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parseCode.ts{ts:line-numbers}[Code]

:::

## Using Rust
In **Rust**, the `parse` method allows you to take raw instruction strings and convert them into the internal VM representation, making it easy to bridge human-written scripts with your Rust-based VM backend.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parse_code.rs{rust:line-numbers}[Code]

:::

::: info
**Capability Required**: no specific capability
:::
