use lightvm::{
  LightVM,
  types::{capability::Capability, runtime_config::RuntimeConfig, vmconfig::VmConfig},
};
use std::time::Duration;

fn config() -> VmConfig {
  VmConfig {
    caps: vec![Capability::Control, Capability::Debug],
    runtime_config: Some(RuntimeConfig { nightly: true }),
    ..Default::default()
  }
}

pub fn run_case(name: &str, raw: &str, export: Option<&str>, optimize: bool) {
  let mut tools_vm = LightVM::new(config());
  let bytecode = if optimize {
    tools_vm.tools().optimize_bytecode(raw)
  } else {
    serde_json::from_str(raw).expect("valid raw bytecode")
  };
  let benchmark_name = format!("{}_{}", name, if optimize { "optimize" } else { "normal" });
  let benchmark = tools_vm
    .tools()
    .bench(&benchmark_name)
    .expect("benchmark requires debug capability")
    .target_time(Duration::from_millis(1))
    .samples(3);

  let setup = || {
    let mut vm = LightVM::new(config());
    vm.load_internal(bytecode.to_string())
      .expect("valid benchmark bytecode");
    if let Some(name) = export {
      assert!(
        vm.exported.contains(name),
        "missing exported function: {name}"
      );
    }
    vm
  };
  let mut preflight = setup();
  if let Some(name) = export {
    let result = preflight
      .export(name.to_string())
      .call(&mut preflight, vec![5.into(), 6.into()]);
    assert_ne!(format!("{result:?}"), "Undefined");
    if optimize {
      let mut baseline = LightVM::new(config());
      baseline
        .load_internal(raw.to_string())
        .expect("valid raw bytecode");
      let expected = baseline
        .export(name.to_string())
        .call(&mut baseline, vec![5.into(), 6.into()]);
      assert_eq!(result, expected, "optimized result differs: {name}");
    }
  } else {
    let output = preflight.run(None);
    let actual: serde_json::Value = serde_json::from_str(&output).expect("successful execution");
    assert_eq!(actual["status"], "success");
    if optimize {
      let mut baseline = LightVM::new(config());
      baseline
        .load_internal(raw.to_string())
        .expect("valid raw bytecode");
      let expected: serde_json::Value =
        serde_json::from_str(&baseline.run(None)).expect("successful raw execution");
      assert_eq!(
        actual["result"], expected["result"],
        "optimized result differs: {name}"
      );
    }
  }

  if let Some(name) = export {
    // Timed operation: an exported-function call; setup rebuilds the VM outside the timed window.
    benchmark.run(
      || {
        let vm = setup();
        let function = vm.export(name.to_string());
        (vm, function)
      },
      |(vm, function)| function.call(vm, vec![5.into(), 6.into()]),
    );
  } else {
    // Timed operation: vm.run(None); every call starts with a fresh VM execution context.
    benchmark.run(setup, |vm| vm.run(None));
  }
}
