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
  }).set_max_io(5000000).set_max_ticks(200).set_max_stack_size(0).with_nightly(false).with_backtrace(false).with_explain(false).with_hint(true);
  
  let raw = r#"[
    ["push", 138],
    ["jump", 0]
  ]"#;
  let str = r#"
  push 5; ;; IP=0
  push 5; ;; IP=1
  add int; ;; IP=2
  println; ;; IP=3
  "#;
  let optimized_json = vm.tools().optimize_bytecode(raw);
  
  vm.load(optimized_json);

  let res = vm.run(None);
  let parsed: serde_json::Value = serde_json::from_str(&res).expect("Failed to parse VM result");
  assert_eq!(parsed["status"], "error", "Expected VM to fail due to tick limit");
  let error_msg = parsed["message"].as_str().expect("Expected error message");
  assert!(error_msg.contains("TickLimitExceeded"), "Expected TickLimitExceeded error, got: {}", error_msg);
  vm.halt();
  vm.run(None); // will not be executed
  println!("The VM has been terminated.");
  vm.on(VmEvent::Halt, |payload| {
    println!("Halted: {}", payload);
  });
  
  /*println!("===> Execution finished <===");
  println!("Output: {:?}", res);*/
}
