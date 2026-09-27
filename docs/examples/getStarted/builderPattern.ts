import { LightVM, Capability, TimeBudget } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
  .setMaxTicks(1_000_000)
  .setTimeBudget(TimeBudget.Cheap);
