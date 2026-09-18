import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Debug] });

console.log(vm.info());
