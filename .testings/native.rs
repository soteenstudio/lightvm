/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::{vmconfig::VmConfig, capability::Capability, vmevent::VmEvent, compile_config::CompileConfig, target_arch::TargetArch, file_type::FileType}};  

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe, Capability::Unsafe],
    ..Default::default()
  }).set_max_io(5000000).set_max_ticks(200).set_max_stack_size(0).with_nightly(false).with_backtrace(false).with_explain(false).with_hint(true);
  
  let raw = r#"[
    ["push", 283892733993939],
    ["push", 83838],
    ["add", ""],
    ["println"]
  ]"#;
  let optimized_json = vm.tools().optimize_bytecode(raw);
  
  println!("{}", optimized_json);
  vm.load(optimized_json);
  
  vm.run(None);
  vm.compile(CompileConfig {
    target_arch: TargetArch::AArch64,
    file_type: FileType::Binary,
    path: "./test"
  });

  /*let assembly_result = vm.compile(CompileConfig {
    target_arch: TargetArch::AArch64,
    file_type: FileType::Assembly,
    path: "./test"
  });
  let assembly_parsed: serde_json::Value = serde_json::from_str(&assembly_result).expect("Failed to parse assembly compile result");
  assert_eq!(assembly_parsed["status"], "success", "Assembly compilation failed: {}", assembly_result);,*/
  
  /*println!("===> Execution finished <===");
  println!("Output: {:?}", res);*/
}
