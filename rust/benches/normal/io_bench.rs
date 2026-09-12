/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::{capability::Capability, vmconfig::VmConfig}};

fn config() -> VmConfig {
  VmConfig {
    caps: vec![Capability::Control, Capability::Debug, Capability::Observe],
    ..Default::default()
  }
}

fn main() {
  let mut vm = LightVM::new(config());
  let raw = r#"[["push", "Hello from LightVM!"], ["println"]]"#;
  let benchmark = vm.tools().bench("io_bench").expect("benchmark requires debug capability");
  benchmark.run(
    || {
      let mut vm = LightVM::new(config());
      vm.load(raw.into());
      vm
    },
    |vm| {
      let _sink = std::io::sink();
      vm.run(None)
    },
  );
}
