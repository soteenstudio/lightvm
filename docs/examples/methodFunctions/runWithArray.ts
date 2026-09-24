import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Control] });
const tools = vm.tools();

const raw = [
  ['push', 5],
  ['val', 'x'],
  ['set', 'x'],
  ['get', 'x'],
  ['println']
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized).run();
