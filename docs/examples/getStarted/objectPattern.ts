import { LightVM, Capability } from 'lightvm';

const vm = new LightVM({
  caps: [Capability.Observe, Capability.Control],
  runtimeConfig: {
    nightly: false // To allow nightly features (default: false)
  },
  errorOptions: {
    backtrace: false, // To display backtrace details in error messages (default: false)
    explain: false, // To display a more detailed hint in the error message (default: false)
    hint: true // To display a hint on error messages (default: true)
  }
});

const tools = vm.tools();