vm.tools()
  .bench('test_bench')
  .bytes(512)
  .samples(20)
  .targetTime(100)
  .run(
    () => [0.5, 6.7, 8.9],
    (state) => vm.tools().blackBox(state),
  );
