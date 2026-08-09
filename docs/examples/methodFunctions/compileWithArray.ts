import { TargetArch, FileType } from "lightvm";

const raw = [
  ['push', 5],
  ['val', 'x'],
  ['set', 'x'],
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized)
  .compile({
    targetArch: TargetArch.AArch64,
    fileType: FileType.Binary,
    path: "./bin/output",
  });
