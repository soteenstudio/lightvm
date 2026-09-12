# Capabilities

A `Capability` grants a host operation permission to access a protected `LightVM` function. Pass the required values in `VMConfig.caps`. TypeScript uses `Observe` by default.

| Capability | Numeric value | Protected operations |
| --- | ---: | --- |
| `Control` | `0` | `run`, `compile`, `embedded`, function exports, `provide`, clearing outputs, and bytecode optimization |
| `Observe` | `1` | `inspect`, reading outputs, and variable exports |
| `Debug` | `2` | Benchmarks |
| `Unsafe` | `3` | `halt` |

## Configuration

::: code-group

<<< @/examples/capsConfig.ts{ts:line-numbers}[TypeScript]

<<< @/examples/caps_config.rs{rs:line-numbers}[Rust]

:::

Missing capabilities cause the protected operation to fail. Capabilities do not replace resource limits in `SecurityConfig`; configure both for untrusted bytecode.

::: warning
Grant only the capabilities the host application needs. `Unsafe` permits an external halt but does not enable `SecurityConfig.unsafeMode`.
:::
