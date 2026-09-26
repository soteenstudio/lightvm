use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control],
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = r#"[
    ["push", 5],
    ["val", "x"],
    ["set", "x"],
    ["get", "x"],
    ["println"]
  ]"#;

  let optimized = tools.optimize_bytecode(raw);
  vm.load(optimized.clone())
    .run(None);
}
