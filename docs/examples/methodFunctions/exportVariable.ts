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
