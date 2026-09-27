use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Unsafe],
    ..Default::default()
  });

  vm.halt();
  vm.run(None); // will not be executed

  println!("The VM has been terminated.");
}
