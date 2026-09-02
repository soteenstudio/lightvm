use lightvm::vmevent::VmEvent;

vm.on(VmEvent::Tick, |data| {
  println!("Event: {:?}", data.event);
  println!("Payload: {:?}", data.payload);
});
vm.run();
