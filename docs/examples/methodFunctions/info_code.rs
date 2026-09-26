use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![],
    ..Default::default()
  });

  println!("{}", vm.info());
}
