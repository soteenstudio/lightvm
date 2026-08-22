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
  }).set_max_io(5000000).set_max_ticks(1000).set_max_stack_size(0).with_nightly(true).with_backtrace(false).with_explain(false).with_hint(true).set_time_budget(TimeBudget::Cheap);
  
  let raw = r#"[
    ["jump", 7],
    ["func", "add", 2, 2, 6, "a", "b"],
    ["get", "a"],
    ["get", "b"],
    ["add", "int"],
    ["return"],
    ["stop"],
    ["export", "add"],
    ["val", "x"],
    ["push", 5],
    ["set", "x"],
    ["export", "x"]
  ]"#;
  let optimized_json = vm.tools().optimize_bytecode(raw);
  println!("{}", optimized_json);
  
  vm.load(optimized_json);
  vm.run(None);
  //let mut add_func = vm.export("add".to_string());
  let mut x_var = vm.export("x".to_string());
  //let result = add_func(vec![5.into(), 6.into()]);
  //println!("Result: {:?}", result);
  println!("{:?}", x_var(vec![]));
}
