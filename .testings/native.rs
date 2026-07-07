/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::{vmconfig::VmConfig, runtime_config::RuntimeConfig, error_options::ErrorOptions, capability::Capability}};  

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    runtime_config: RuntimeConfig {
      nightly: false,
    },
    error_options: ErrorOptions {
      backtrace: false,
      explain: false,
      hint: false
    }
  });
  
  let raw = r#"[
    ["get"]
  ]"#;
  let optimized_json = vm.tools().optimize_bytecode(raw);
  
  vm.load(optimized_json);
  
  let _res = vm.run(None);
  
  /*println!("===> Execution finished <===");
  println!("Output: {:?}", res);*/
}
