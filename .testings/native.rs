/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, RunOptions, types::{vmconfig::VmConfig, capability::Capability, time_budget::TimeBudget}};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe, Capability::Unsafe],
    ..Default::default()
  }).set_max_io(5000000).set_max_ticks(1000).set_max_stack_size(0).with_nightly(true).with_backtrace(false).with_explain(false).with_hint(true).set_time_budget(TimeBudget::Cheap);
  
  let raw = r#"[
    ["push", 5],
    ["push", 6],
    ["push", 7],
    ["make_array", 3],
    ["push", 8],
    ["push", 9],
    ["push", 10],
    ["make_array", 3],
    ["dot", "int"],
    ["return"]
  ]"#;
  let tools = vm.tools();
  let optimized_json = tools
    .optimize_bytecode(raw);
  vm.load(optimized_json);
  let result: serde_json::Value = serde_json::from_str(&vm.run(Some(RunOptions {
    capture_return: true,
    ..Default::default()
  }))).expect("expected VM result JSON");
  assert_eq!(result["result"]["value"], 164);
}
