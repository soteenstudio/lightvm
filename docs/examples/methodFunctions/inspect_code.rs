use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let vm = LightVM::new(VmConfig {
    caps: vec![Capability::Observe],
    ..Default::default()
  });

  let report = vm.inspect();

  println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
