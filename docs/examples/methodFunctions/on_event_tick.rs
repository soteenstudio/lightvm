use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, vmevent::VmEvent};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![],
    ..Default::default()
  });

  vm.on(VmEvent::Tick, |data| {
    println!("Event: {:?}", data.event);
    println!("Payload: {:?}", data.payload);
  });
  vm.run(None);
}
