# TypeScript Configuration
This page provides a comprehensive reference for all available configuration options, limits, and security controls when initializing a `LightVM` instance.

## Full Configuration Example
The following example demonstrates how to configure all available limits, safety flags, and error options using both the builder and object patterns.

::: code-group
<<< @/examples/configuration-reference/builderPattern.ts{ts:line-numbers}[Builder Pattern]
<<< @/examples/configuration-reference/objectPattern.ts{ts:line-numbers}[Object Pattern]
:::

## Configuration Options Breakdown

| Method / Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `caps` | `Capability[]` | `[]` | Specifies the active capabilities granted to the virtual machine. |
| `setMaxIo` / `maxIo` | `number` | `100` | Maximum number of I/O operations allowed during execution. |
| `setMaxImport` / `maxImport` | `number` | `3` | Maximum number of allowed module imports. |
| `setMaxAlloc` / `maxAlloc` | `number` | `50` | Maximum number of memory allocations allowed. |
| `setMaxCall` / `maxCall` | `number` | `200` | Maximum number of nested function calls allowed. |
| `setMaxJump` / `maxJump` | `number` | `100` | Maximum number of control flow jumps allowed. |
| `setMaxTicks` / `maxTicks` | `number` | `1_000_000` | Maximum number of execution ticks before stopping to prevent infinite loops. |
| `setMaxStackSize` / `maxStackSize` | `number` | `128` | Maximum number of items the evaluation stack can hold. |
| `setAllowedImports` / `allowedImports` | `string[]` | `[]` | Whitelist of module names that are permitted to be imported. |
| `setTimeBudget` / `timeBudget` | `TimeBudget` | `TimeBudget.Cheap` | Sets the execution time budget tier limit. |
| `withUnsafeMode` / `unsafeMode` | `boolean` | `false` | Enables or disables system-level unsafe operations. |
| `withNightly` / `nightly` | `boolean` | `false` | Allows the usage of experimental nightly features. |
| `withBacktrace` / `backtrace` | `boolean` | `false` | Displays internal backtrace details in error messages. |
| `withExplain` / `explain` | `boolean` | `false` | Displays a more detailed explanatory hint in error messages. |
| `withHint` / `hint` | `boolean` | `true` | Displays general usage hints on error messages. |
| `withDiagnosticLinks` / `diagnosticLinks` | `boolean` | `true` | Includes metadata links pointing to error-code documentation. |

::: info Related Documentation
Need to check individual error codes mentioned in error messages? Visit the [Error Code Reference](/api-reference/error-codes/lvm001-code).
:::