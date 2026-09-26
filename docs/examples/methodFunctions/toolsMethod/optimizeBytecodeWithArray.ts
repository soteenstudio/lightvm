import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Control] });
const tools = vm.tools();

const raw = [
  ['push', 5],
  ['val', 'x'],
  ['set', 'x'],
];

const optimized = tools.optimizeBytecode(raw);

console.log(optimized);
