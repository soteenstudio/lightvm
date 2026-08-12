use std::time::Duration;

vm.tools().bench("test_bench")
  .bytes(1024)
  .samples(20)
  .target_time(Duration::from_millis(100))
  .run(
    || vec![0.5, 6.7, 8.9],
    |state| std::hint::black_box(state),
  );