use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = serde_json::json!([
    ["push", 42],
    ["stop"]
  ]);

  let optimized = tools.optimize_bytecode(raw);
  vm.load(optimized.clone());

  let result = vm.embedded();

  println!("value: {}", result["value"]);
  println!("outputs: {}", result["outputs"]);
  println!("halted: {}", result["halted"]);
}
