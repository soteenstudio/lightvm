# Compile Method
After initializing the VM and setting up your environment, you can process and compile your bytecode configuration into a target binary.

## Using TypeScript
For **TypeScript**, you can pass raw instruction arrays directly to the loader, apply optimization tools, and set up your compilation configurations seamlessly.

::: code-group

<<< @/examples/methodFunctions/compileWithArray.ts{ts:line-numbers}[With Array]

:::

## Using Rust
In **Rust**, you typically work with raw instruction strings, optimize them using the helper tools, and pass a `CompileConfig` struct containing your `TargetArch` and `FileType` directly into the `.compile()` method.

::: code-group

<<< @/examples/methodFunctions/compile_with_raw_string.rs{rust:line-numbers}[With Raw String]

<<< @/examples/methodFunctions/compile_with_serde.rs{rust:line-numbers}[With Serde]

:::

## Target Architecture
| Architecture | Compile |
|--------------|---------|
| AArch64      | ✓       |

## File Type
| Type | Description |
|------|-------------|
| `Assembly` |
| `Binary` |
::: info
**Capability Required**: `Control`
:::
