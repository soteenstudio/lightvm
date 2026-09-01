# Embedded Method

The `.embedded()` method executes the program currently loaded in the VM and returns a `VMResult`.

Before execution, the method clears outputs from the previous embedded execution. The returned `VMResult.outputs` therefore contains only output produced by the current execution.

## Using TypeScript

Create the VM with both `Control` and `Observe`, load a program, and call `.embedded()`:

::: code-group

<<< @/examples/methodFunctions/embeddedCode.ts{ts:line-numbers}[Embedded Execution]

:::

## Result

`VMResult` contains these fields:

- `value`: The program's defined return value. It is `null` when the program returns no value or when the VM is halted.
- `outputs`: Output produced only by the current embedded execution.
- `halted`: The VM's current halt state.

::: info
**Capabilities Required**: `Control` executes the program, and `Observe` retrieves its outputs.
:::

## Errors

Native `.embedded()` returns `{ status: "error", message }` when execution fails. The N-API and WebAssembly bindings propagate failures through their binding error mechanisms.
