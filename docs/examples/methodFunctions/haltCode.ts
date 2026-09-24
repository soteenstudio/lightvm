import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Unsafe] });

vm.halt();
vm.run(); // will not be executed
console.log('The VM has been terminated.');
