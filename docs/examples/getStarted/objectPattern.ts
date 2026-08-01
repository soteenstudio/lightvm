import { LightVM, Capability } from 'lightvm';

const vm = new LightVM({
  caps: [Capability.Observe, Capability.Control],
  runtimeConfig: {
    nightly: false, // Allow nightly features (default: false)
  },
  errorOptions: {
    backtrace: false, // Display backtrace details in error messages (default: false)
    explain: false, // Display a more detailed hint in the error message (default: false)
    hint: true, // Display a hint on error messages (default: true)
  },
  securityConfig: {
    maxIo: 100, // Maximum number of I/O operations allowed (default: 100)
    maxImport: 3, // Maximum number of allowed module imports (default: 3)
    maxAlloc: 50, // Maximum number of memory allocations allowed (default: 50)
    maxCall: 200,  // Maximum number of nested function calls allowed (default: 200)
    maxJump: 100,  // Maximum number of control flow jumps allowed (default: 100)
    maxTicks: 1_000_000,  // Maximum number of execution ticks before stopping (default: 1,000,000)
    allowedImports: ["math", "time", "utils"], // Whitelist of modules that can be imported
    unsafeMode: false // Enable or disable system-level unsafe operations (default: false)
  }
});

const tools = vm.tools();
