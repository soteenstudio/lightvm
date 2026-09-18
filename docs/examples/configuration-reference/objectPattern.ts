import { LightVM, Capability, TimeBudget } from 'lightvm';

const vm = new LightVM({
  caps: [Capability.Observe, Capability.Control],
  runtimeConfig: {
    nightly: false,
  },
  errorOptions: {
    backtrace: false,
    explain: false,
    hint: true,
    diagnosticLinks: false,
  },
  securityConfig: {
    maxIo: 100,
    maxImport: 3,
    maxAlloc: 50,
    maxCall: 200,
    maxJump: 100,
    maxTicks: 1_000_000,
    maxStackSize: 128,
    allowedImports: ['math', 'time', 'utils'],
    timeBudget: TimeBudget.Cheap,
    unsafeMode: false,
  },
});
