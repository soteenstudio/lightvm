import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({
  caps: [Capability.Control, Capability.Observe],
  runtimeConfig: { nightly: true },
});
const tools = vm.tools();

const raw = [
  ['val', 'score'],
  ['push', '2'],
  ['set', 'score'],
  ['export', 'score'],
];

const optimized = tools.optimizeBytecode(raw);
vm.load(optimized);

const scoreVar = vm.export('score');

console.log(scoreVar.call());
