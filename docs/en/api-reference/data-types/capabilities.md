# Capabilities

A `Capability` grants a host operation permission to access a protected `LightVM` function. Pass the required values in `VMConfig.caps`. TypeScript uses `Observe` by default.

| Capability | Numeric value | Protected operations |
| --- | ---: | --- |
| `Control` | `0` | `run`, `compile`, `embedded`, function exports, `provide`, clearing outputs, and bytecode optimization |
| `Observe` | `1` | `inspect`, reading outputs, and variable exports |
| `Debug` | `2` | Benchmarks |
| `Unsafe` | `3` | `halt` |

::: warning
Failing to provide the required capabilities will cause protected host operations to fail execution or trigger runtime errors. Always ensure you grant the minimum necessary capability permissions required by your application.
:::
