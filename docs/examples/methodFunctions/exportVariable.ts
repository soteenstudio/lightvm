const raw = [
<<<<<<< Updated upstream
  ['val', 'x'],
  ['push', 5],
  ['set', 'x'],
  ['export', 'x'],
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized);
const x = vm.export('x');
console.log(x.call());
=======
  ['val', 'score'],
  ['push', '2'],
  ['set', 'score'],
  ['export', 'score']
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized);
const scoreVar= vm.export('score');
console.log(scoreVar());
>>>>>>> Stashed changes
