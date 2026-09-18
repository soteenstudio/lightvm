use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = serde_json::json!([
    ["push", 5],
    ["val", "x"],
    ["set", "x"]
  ]);

  let stringify = tools.stringify_ltc(raw);

  println!("{:#}", stringify.clone());
}
