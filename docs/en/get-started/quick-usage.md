# Quick Usage

Create and configure a `LightVM` instance. Complete [Installation](/get-started/installation) first.

## Using TypeScript
For TypeScript projects, configure the constructor or use the chainable configuration methods.

::: code-group

<<< @/examples/getStarted/builderPattern.ts{ts:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/objectPattern.ts{ts:line-numbers}[Object Pattern]

:::

## Using Rust
For Rust projects, configure `VmConfig` before creating the VM.

::: code-group

<<< @/examples/getStarted/builder_pattern.rs{rust:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/object_pattern.rs{rust:line-numbers}[Object Pattern]

:::

## Diagnostic links

Formatted VM errors include a `documentation:` metadata row by default. Its URL is derived from the error's `LVM` code and points to the matching page in the [error-code reference](/api-reference/error-codes/lvm001-code).

Disable the row through the constructor configuration or the fluent API:

::: code-group

```rust [Native configuration]
let vm = LightVM::new(VmConfig {
  error_options: Some(ErrorOptions {
    diagnostic_links: false,
    ..Default::default()
  }),
  ..Default::default()
});

let vm = LightVM::new(VmConfig::default())
  .with_diagnostic_links(false);
```

```ts [Node.js]
const vm = new LightVM({
  errorOptions: {
    backtrace: false,
    explain: false,
    hint: true,
    diagnosticLinks: false,
  },
});

vm.withDiagnosticLinks(false);
```

```ts [WASM]
const vm = new LightVM({
  caps: [],
  errorOptions: { diagnosticLinks: false },
});

vm.withDiagnosticLinks(false);
```

:::

The same setting is retained by `tools()` and applies to errors from tool operations, including bytecode optimization.

## Expected result

You have a configured VM instance ready to load bytecode. Continue with the [Run Method](/api-reference/method-functions/run-method), or review [Capabilities](/api-reference/capabilities) before granting access.

::: info Related API Reference
`TimeBudget` limits bytecode optimization time. Use `SecurityConfig.maxTicks` to limit execution work.
:::
