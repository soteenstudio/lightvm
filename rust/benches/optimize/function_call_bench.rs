/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{
  LightVM,
  types::{capability::Capability, runtime_config::RuntimeConfig, vmconfig::VmConfig},
};

fn config() -> VmConfig {
  VmConfig {
    caps: vec![Capability::Control, Capability::Debug, Capability::Observe],
    runtime_config: Some(RuntimeConfig { nightly: true }),
    ..Default::default()
  }
}

fn main() {
  let mut vm = LightVM::new(config());
  let raw = r#"[
    ["jump", 7],
    ["func", "add", 2, 2, 6, "a", "b"],
    ["get", "a"],
    ["get", "b"],
    ["add", "int"],
    ["return"],
    ["stop"],
    ["export", "add"]
  ]"#;
  let tools = vm.tools();
  let optimized = tools.optimize_bytecode(raw);
  let benchmark = tools
    .bench("function_call_bench")
    .expect("benchmark requires debug capability");
  benchmark.run(
    || {
      let mut vm = LightVM::new(config());
      vm.load(optimized.clone());
      let function = vm.export("add".to_string());
      (vm, function)
    },
    |(vm, function)| function.call(vm, vec![5.into(), 6.into()]),
  );
}
