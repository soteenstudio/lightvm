/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

import { LightVM, Capability } from '../../../dist/index.min.mjs';
function runBenchmark() {
  const vm = new LightVM({
    caps: [Capability.Control, Capability.Debug, Capability.Observe],
  });
  const raw = [['push', 'Hello from LightVM!'], ['println']];
  const tools = vm.tools();
  const optimized = tools.optimizeBytecode(raw);

  tools.bench('io_bench').run(
    () => {
      const benchmarkVm = new LightVM({
        caps: [Capability.Control, Capability.Debug, Capability.Observe],
      });
      benchmarkVm.load(optimized);
      return benchmarkVm;
    },
    (benchmarkVm) => benchmarkVm.run(),
  );
}
runBenchmark();
