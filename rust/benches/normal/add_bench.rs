/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use lightvm::{LightVM, types::capability::Capability};
use std::time::Duration;
fn bench_vm_execution(c: &mut Criterion) {
  let mut vm = LightVM::new(vec![Capability::Control, Capability::Observe]);
  let raw = r#"[
    ["val", "x"],
    ["push", 5],
    ["push", 8],
    ["add", "i16"],
    ["set", "x"]
  ]"#;
  vm.load(raw.into());
  let mut group = c.benchmark_group("LightVM Execution");
  group.bench_function("add_bench", |b: &mut Bencher| {
    b.iter(|| vm.run(None));
  });
  group.finish();
}
fn custom_config() -> Criterion {
  Criterion::default()
    .sample_size(300)
    .measurement_time(Duration::from_secs(15))
    .warm_up_time(Duration::from_secs(3))
}
criterion_group! {
  name = benches;
  config = custom_config();
  targets = bench_vm_execution
}
criterion_main!(benches);
