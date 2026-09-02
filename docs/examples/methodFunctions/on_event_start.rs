use lightvm::vmevent::VmEvent;

vm.on(VmEvent::Start, |data| {
  println!("Event: {:?}", data.event);
  println!("Payload: {:?}", data.payload);
});
vm.run();
