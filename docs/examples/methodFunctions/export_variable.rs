use lightvm::LightVM;
use lightvm::types::{
  capability::Capability, runtime_config::RuntimeConfig, vmconfig::VmConfig,
};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    runtime_config: Some(RuntimeConfig { nightly: true }),
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = r#"[
    ["val", "score"],
    ["push", "2"],
    ["set", "score"],
    ["export", "score"]
  ]"#;

  let optimized = tools.optimize_bytecode(raw);
  vm.load(optimized);

  let score_variable = vm.export("score".to_string());

  println!("{:?}", score_variable.call(&mut vm, vec![]));
}
