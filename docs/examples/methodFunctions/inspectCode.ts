import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Control] });

const report = vm.inspect();

console.log(report);
