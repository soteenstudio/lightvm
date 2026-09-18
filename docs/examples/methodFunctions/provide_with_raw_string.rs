use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });

  vm.provide(serde_json::json!({
    "name": "John Doe",
    "force": 2021
  }));

  let raw = r#"[
    ["get", "name"],
    ["println"],
    ["get", "force"],
    ["println"]
  ]"#;
}
