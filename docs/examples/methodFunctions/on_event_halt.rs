use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig, vmevent::VmEvent};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });

  vm.on(VmEvent::Halt, |data| {
    println!("Event: {:?}", data.event);
    println!("Payload: {:?}", data.payload);
  });
  vm.halt();
  vm.run(None); // will not be executed
}
