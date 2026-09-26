# Capabilities

A `Capability` grants a host operation permission to access a protected `LightVM` function. Pass the required values in `VMConfig.caps`. TypeScript uses `Observe` by default.

| Capability | Numeric value | Protected operations |
| --- | ---: | --- |
| `Control` | `0` | `run`, `compile`, `embedded`, function exports, `provide`, clearing outputs, and bytecode optimization |
| `Observe` | `1` | `inspect`, reading outputs, and variable exports |
| `Debug` | `2` | Benchmarks, reading panic records, and clearing panic records |
| `Unsafe` | `3` | `halt`, which stops the VM |

`unsafeMode` (or `unsafe_mode` in **Rust**) in disables security checks such as import restrictions and resource quotas for I/O, imports, allocations, calls, and jumps, but does not grant capability permissions. Bounds verification remains enabled. Calling `halt` still requires the `Unsafe` capability in `VMConfig.caps`, even with this mode enabled.

::: warning
Failing to provide the required capabilities will cause protected host operations to fail execution or trigger runtime errors. Always ensure you grant the minimum necessary capability permissions required by your application.
:::
