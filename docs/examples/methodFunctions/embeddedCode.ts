import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({
  caps: [Capability.Control, Capability.Observe],
});

vm.load([
  ['push', 42],
  ['stop'],
]);

const result = vm.embedded();

console.log(result);
// { value: 42, outputs: [], halted: false }
