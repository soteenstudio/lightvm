/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

import { Capability, LightVM } from '../../../dist/index.min.mjs';

const config = () => ({
  caps: [Capability.Control, Capability.Debug, Capability.Observe],
  runtimeConfig: { nightly: true },
});
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
const vm = new LightVM(config());
const tools = vm.tools();
const optimized = tools.optimizeBytecode(raw);

tools.bench('function_call_bench').run(
  () => {
    const benchmarkVm = new LightVM(config()).load(optimized);
    return { vm: benchmarkVm, function: benchmarkVm.export('add') };
  },
  (state) => state.function.call(5, 6),
);
