import { LightVM, Capability } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
  .withNightly(false) // To allow nightly features (default: false)
  .withBacktrace(false) // To display backtrace details in error messages (default: false)
  .withExplain(false) // To display a more detailed hint in the error message (default: false)
  .withHint(true); // To display a hint on error messages (default: true)

const tools = vm.tools();
