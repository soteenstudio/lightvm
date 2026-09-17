use lightvm::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });

  println!("{}", vm.info());
}
