use lightvm::LightVM;
use lightvm::{vmconfig::VmConfig, types::capability::Capability};

fn main() {
  let vm = LightVM::new(VmConfig {
    caps: [Capability.Observe, Capability.Control]
  });
}
