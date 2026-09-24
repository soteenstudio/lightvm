import { Capability, LightVM } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Debug] });
const tools = vm.tools();

const raw = [
  ['push', 5],
  ['val', 'x'],
  ['set', 'x'],
];
const stringify = tools.stringifyLTC(raw);
console.log(stringify);
