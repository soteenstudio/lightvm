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
});

const vm = new LightVM(config());
const raw = [
  ['push', 5],
  ['push', 8],
  ['add', 'i16'],
  ['val', 'x'],
  ['push', 9],
  ['set', 'x'],
];

vm.tools()
  .bench('dead_code_bench')
  .run(
    () => new LightVM(config()).load(raw),
    (benchmarkVm) => benchmarkVm.run(),
  );
