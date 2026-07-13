const raw = [
  ['push', 5],
  ['val', 'x'],
  ['set', 'x'],
];
const optimized = tools.optimizeBytecode(raw);
console.log(optimized);
