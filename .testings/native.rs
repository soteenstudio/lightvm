/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::{vmconfig::VmConfig, capability::Capability, time_budget::TimeBudget}};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe, Capability::Unsafe],
    ..Default::default()
  }).set_max_io(5000000).set_max_ticks(200).set_max_stack_size(0).with_nightly(false).with_backtrace(false).with_explain(false).with_hint(true).set_time_budget(TimeBudget::Cheap);
  
  let raw = r#"[
    ["push", 5],
    ["asinh", "flt"],
    ["println"],
    ["jump", 0]
  ]"#;
  let optimized_json = vm.tools().optimize_bytecode(raw);
  println!("{}", optimized_json);
  
  vm.load(optimized_json);
  vm.run(None);
}
