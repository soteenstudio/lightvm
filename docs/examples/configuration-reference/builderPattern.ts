import { LightVM, Capability, TimeBudget } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
  .setMaxIo(100)
  .setMaxImport(3)
  .setMaxAlloc(50)
  .setMaxCall(200)
  .setMaxJump(100)
  .setMaxTicks(1_000_000)
  .setMaxStackSize(128)
  .setAllowedImports(['math', 'time', 'utils'])
  .setTimeBudget(TimeBudget.Cheap)
  .withUnsafeMode(false)
  .withNightly(false)
  .withBacktrace(false)
  .withExplain(false)
  .withHint(true)
  .withDiagnosticLinks(false);
