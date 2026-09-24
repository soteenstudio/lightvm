use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control],
    ..Default::default()
  });
  let tools = vm.tools();

  vm.provide(serde_json::json!({
    "name": "John Doe",
    "force": 2021
  }));

  let raw = serde_json::json!([
    ["get", "name"],
    ["println"],
    ["get", "force"],
    ["println"]
  ]);

  let optimized = tools.optimize_bytecode(raw);
  vm.load(optimized)
    .run(None);
}
