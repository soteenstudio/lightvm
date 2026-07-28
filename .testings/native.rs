/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::{vmconfig::VmConfig, capability::Capability, vmevent::VmEvent}};  

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe, Capability::Unsafe],
    ..Default::default()
  }).with_nightly(false).with_backtrace(false).with_explain(false).with_hint(true);
  
  let raw = r#"[
    ["val", "x"],
    ["push", 5],
    ["set", "x"],
    ["get", "x"],
    ["println"]
  ]"#;
  let str = r#"
  push 5; ;; IP=0
  push 5; ;; IP=1
  add int; ;; IP=2
  println; ;; IP=3
  "#;
  println!("result {}", vm.tools().parse_ltc_array(str));
  let optimized_json = vm.tools().optimize_bytecode(raw);
  
  println!("{}", optimized_json);
  vm.load(optimized_json);
  
  let res = vm.run(None);
  println!("Res: {}", res);
  vm.halt();
  vm.run(None); // will not be executed
  println!("The VM has been terminated.");
  vm.on(VmEvent::Halt, |payload| {
    println!("Halted: {}", payload);
  });
  
  /*println!("===> Execution finished <===");
  println!("Output: {:?}", res);*/
}
