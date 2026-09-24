import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Control] });
const tools = vm.tools();

vm.provide({
  name: 'John Doe',
  force: 2021,
});

const raw = [
  ['get', 'name'],
  ['println'],
  ['get', 'force'],
  ['println']
];

const optimized = tools.optimizeBytecode(raw);
vm.load(optimized).run();
