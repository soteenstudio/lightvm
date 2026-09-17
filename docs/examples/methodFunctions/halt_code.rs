use lightvm::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });

  vm.halt();
  vm.run(None); // will not be executed
  
  println!("The VM has been terminated.");
}
