/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

import { Bench } from 'tinybench';
import { LightVM, Capability } from "../../../dist/index.min.mjs";
async function runBenchmark() {
  const bench = new Bench();
  const vm = new LightVM([Capability.Observe, Capability.Control]);
  const raw = [["val", "x"], ["push", 5], ["set", "x"]];
  const tools = vm.tools();
  const optimized = tools.optimizeBytecode(raw);
  vm.load(optimized);
  bench.add('add_bench', () => {
    vm.run();
  });
  await bench.run();
  bench.table().forEach((row) => {
    console.log(`Task: ${row['Task name']} | Avg Latency: ${row['Latency avg (ns)']}`);
  });
}
runBenchmark().catch(console.error);
