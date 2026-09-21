# Rust Configuration
This page provides a comprehensive reference for all available configuration options, limits, and security controls when initializing a `LightVM` instance.

## Full Configuration Example
The following example demonstrates how to configure all available limits, safety flags, and error options using both the builder and object patterns.

::: code-group
<<< @/examples/configuration-reference/builder_pattern.rs{rs:line-numbers}[Builder Pattern]
<<< @/examples/configuration-reference/object_pattern.rs{rs:line-numbers}[Object Pattern]
:::

## Configuration Options Breakdown

| Method / Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `caps` | `Capability[]` | `[]` | Specifies the active capabilities granted to the virtual machine. |
| `set_max_io` / `max_io` | `number` | `100` | Maximum number of I/O operations allowed during execution. |
| `set_max_import` / `max_import` | `number` | `3` | Maximum number of allowed module imports. |
| `set_max_alloc` / `max_alloc` | `number` | `50` | Maximum number of memory allocations allowed. |
| `set_max_call` / `max_call` | `number` | `200` | Maximum number of nested function calls allowed. |
| `set_max_jump` / `max_jump` | `number` | `100` | Maximum number of control flow jumps allowed. |
| `set_max_ticks` / `max_ticks` | `number` | `1_000_000` | Maximum number of execution ticks before stopping to prevent infinite loops. |
| `set_max_stack_size` / `max_stack_size` | `number` | `128` | Maximum number of items the evaluation stack can hold. |
| `set_allowed_imports` / `allowed_imports` | `string[]` | `[]` | Whitelist of module names that are permitted to be imported. |
| `set_time_budget` / `time_budget` | `TimeBudget` | `TimeBudget.Cheap` | Sets the execution time budget tier limit. |
| `with_unsafe_mode` / `unsafe_mode` | `boolean` | `false` | Enables or disables system-level unsafe operations. |
| `with_nightly` / `nightly` | `boolean` | `false` | Allows the usage of experimental nightly features. |
| `with_backtrace` / `backtrace` | `boolean` | `false` | Displays internal backtrace details in error messages. |
| `with_explain` / `explain` | `boolean` | `false` | Displays a more detailed explanatory hint in error messages. |
| `with_hint` / `hint` | `boolean` | `true` | Displays general usage hints on error messages. |
| `with_diagnostic_links` / `diagnostic_links` | `boolean` | `true` | Includes metadata links pointing to error-code documentation. |

::: info Related Documentation
Need to check individual error codes mentioned in error messages? Visit the [Error Code Reference](/api-reference/error-codes/lvm001-code).
:::