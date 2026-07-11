const raw = [
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
];
vm.load(tools.optimizeBytecode(raw))
  .run();