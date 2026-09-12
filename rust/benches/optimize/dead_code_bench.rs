/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::capability::Capability};
fn main() {
  let capabilities = vec![
    Capability::Control,
    Capability::Debug,
    Capability::Observe,
  ];
  let mut vm = LightVM::new(capabilities.clone());
  let raw = r#"[
    ["push", 5],
    ["push", 8],
    ["add", "i16"],
    ["val", "x"],
    ["push", 9],
    ["set", "x"]
  ]"#;
  let tools = vm.tools();
  let optimized_json = tools.optimize_bytecode(raw);
  let benchmark = tools
    .bench("dead_code_bench")
    .expect("benchmark requires debug capability");
  benchmark.run(
    || {
      let mut vm = LightVM::new(capabilities.clone());
      vm.load(optimized_json.clone());
      vm
    },
    |vm| vm.run(None),
  );
}
