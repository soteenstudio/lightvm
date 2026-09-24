import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Control, Capability.Observe] });
const tools = vm.tools();

const raw = [
  ['push', 42],
  ['stop']
];

const optimized = tools.optimizeBytecode(raw);
vm.load(optimized);

const result = vm.embedded();

console.log(result);
