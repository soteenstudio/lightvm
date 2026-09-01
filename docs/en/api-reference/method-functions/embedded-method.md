# Embedded Method

The `.embedded()` method executes the program currently loaded in the VM and returns its defined `value`, current execution `outputs`, and VM `halted` state. An undefined result or halted VM supplies `null` for `value`.

## Using TypeScript
For **TypeScript**, create the VM with `Control` and `Observe`, load a raw bytecode array, and call `.embedded()`.

::: code-group

<<< @/examples/methodFunctions/embeddedCode.ts{ts:line-numbers}[With Array]

:::

## Using Rust
In **Rust**, you can load a serialized bytecode string or a `serde_json` value before calling `.embedded()`.

::: code-group

<<< @/examples/methodFunctions/embedded_with_raw_string.rs{rust:line-numbers}[With Raw String]

<<< @/examples/methodFunctions/embedded_with_serde.rs{rust:line-numbers}[With Serde]

:::

::: info
**Capabilities Required**: `Control` executes the program, and `Observe` retrieves its outputs.
:::

::: tip
Before each embedded execution, `.embedded()` clears outputs left by the prior embedded execution. The returned `outputs` belong to the current execution, `value` contains a defined result or `null` for an undefined result or halted VM, and `halted` reports the VM halt state. Native execution failures return `{ status: "error", message }`, while N-API and WebAssembly propagate failures through their binding error mechanisms.
:::
