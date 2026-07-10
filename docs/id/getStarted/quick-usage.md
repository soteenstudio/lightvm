# Quick Usage
LightVM initialization is **very flexible** and allows you to configure the **capabilities** and **debugging features** according to your application needs.

## Using TypeScript
For **TypeScript-based** projects, you can configure a VM instance with an intuitive builder pattern before accessing the main `tools` interface.

```ts
import { LightVM, Capability } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
  .withNightly(false) // To allow nightly features (default: false)
  .withBacktrace(false) // To display backtrace details in error messages (default: false)
  .withExplain(false) // To display a more detailed hint in the error message (default: false)
  .withHint(true); // To display a hint on error messages (default: true)

const tools = vm.tools();
```

## Using Rust
For **Rust** users, configuration is done through `VmConfig`. You can declaratively set VM capabilities before executing bytecode.

```rust
use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  })
  .with_nightly(false) // To allow nightly features (default: false)
  .with_backtrace(false) // To display backtrace details in error messages (default: false)
  .with_explain(false) // To display a more detailed hint in the error message (default: false)
  .with_hint(true); // To display a hint on error messages (default: true)
  
  let tools = vm.tools();
}
```

::: tip
Get the tools interface. Store this as a constant to reuse it for all upcoming tasks.
:::

## Capabilities
The security and behavior of `LightVM` are managed through a robust capability system. Use the following table to understand which permissions are required for your specific use case:

| Capability | Level | Description |
|------------|-------|-------------|
| Control | Low | Grants permission to start/stop execution and export functions. |
| Observe | Medium | Allows the host to inspect internal states, variable stacks, and metrics. |
| Debug | High | Opens access to verbose internal logs and hidden states for troubleshooting. |
| Unsafe | Critical | Removes safety guards, allowing manual halts and raw memory/process access. |

::: warning Security Notice
Always adhere to the **Principle of Least Privilege**. Only enable the specific capabilities required for your application to ensure a secure and predictable execution environment.
:::