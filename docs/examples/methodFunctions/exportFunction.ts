const raw = [
  ['jump', 7],
  ['func', 'add', 2, 2, 6, 'a', 'b'],
  ['get', 'a'],
  ['get', 'b'],
  ['add', 'int'],
  ['return'],
  ['stop'],
  ['export', 'add'],
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized);
const addFunc = vm.export('add');
console.log(addFunc.call(5, 6));
