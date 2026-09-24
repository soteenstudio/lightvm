import { Capability, FileType, LightVM, TargetArch } from 'lightvm';

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
vm.load(optimized).compile({
  targetArch: TargetArch.AArch64,
  fileType: FileType.Binary,
  path: './bin/output',
});
