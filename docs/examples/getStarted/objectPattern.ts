import { LightVM, Capability, TimeBudget } from 'lightvm';

const vm = new LightVM({
  caps: [Capability.Observe, Capability.Control],
  securityConfig: {
    maxTicks: 1_000_000,
    timeBudget: TimeBudget.Cheap,
  },
});
