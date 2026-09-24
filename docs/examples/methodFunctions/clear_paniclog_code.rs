use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });
  vm.clear_paniclog()?;

  Ok(())
}
