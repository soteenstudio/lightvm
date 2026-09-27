use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig, vmevent::VmEvent};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control],
    ..Default::default()
  });

  vm.on(VmEvent::Start, |data| {
    println!("Event: {:?}", data.event);
    println!("Payload: {:?}", data.payload);
  });
  vm.load(r#"[["stop"]]"#)
    .run(None);
}
