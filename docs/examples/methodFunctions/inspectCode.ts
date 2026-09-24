import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe] });

const report = vm.inspect();

console.log(report);
