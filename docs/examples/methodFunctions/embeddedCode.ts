const raw = [
  ['push', 42],
  ['stop'],
];
const optimized = tools.optimizeBytecode(raw);
vm.load(optimized)
const result = vm.embedded();
console.log(result);
