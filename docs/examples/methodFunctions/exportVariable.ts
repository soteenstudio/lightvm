const raw = [
  ['val', 'x'],
  ['push', 5],
  ['set', 'x'],
  ['export', 'x'],
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized);
const x = vm.export('x');
console.log(x.call());
