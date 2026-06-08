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
fn bench_vm_execution(c: &mut Criterion) {
  let mut vm = LightVM::new(vec![Capability::Control, Capability::Observe]);
  let raw = r#"[["push", "Hello from LightVM!"], ["println"]]"#;
  let optimized_json = LightVM::tools().optimize_bytecode(raw);
  vm.load(optimized_json.clone());
  let mut group = c.benchmark_group("LightVM Execution");
  group.bench_function("io_bench", |b: &mut Bencher| {
    b.iter(|| {
      let _sink = std::io::sink();
      vm.run(None)
    });
  });
  group.finish();
}
criterion_group!(benches, bench_vm_execution);
criterion_main!(benches);
