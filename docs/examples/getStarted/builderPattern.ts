import { LightVM, Capability, TimeBudget } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
  .setMaxIo(100) // Maximum number of I/O operations allowed (default: 100)
  .setMaxImport(3) // Maximum number of allowed module imports (default: 3)
  .setMaxAlloc(50) // Maximum number of memory allocations allowed (default: 50)
  .setMaxCall(200) // Maximum number of nested function calls allowed (default: 200)
  .setMaxJump(100) // Maximum number of control flow jumps allowed (default: 100)
  .setMaxTicks(1_000_000) // Maximum number of execution ticks before stopping (default: 1,000,000)
  .setMaxStackSize(128) // Maximum number of items the stack can hold (default: 128)
  .setAllowedImports(['math', 'time', 'utils']) // Whitelist of modules that can be imported
  .setTimeBudget(TimeBudget.Cheap) // Sets the execution time budget limit to prevent infinite loops (Default: Cheap)
  .withUnsafeMode(false) // Enable or disable system-level unsafe operations (default: false)
  .withNightly(false) // Allow nightly features (default: false)
  .withBacktrace(false) // Display backtrace details in error messages (default: false)
  .withExplain(false) // Display a more detailed hint in the error message (default: false)
  .withHint(true); // Display a hint on error messages (default: true)

const tools = vm.tools();
