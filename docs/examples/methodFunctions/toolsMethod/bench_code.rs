use std::time::Duration;
use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });
  let tools = vm.tools();

  tools.bench("test_bench")
    .unwrap()
    .bytes(1024)
    .samples(20)
    .target_time(Duration::from_millis(100))
    .run(
      || vec![0.5, 6.7, 8.9],
      |state| std::hint::black_box(state),
    );
}
