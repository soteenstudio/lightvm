use lightvm::vmevent::VmEvent;

vm.on(VmEvent::Halt, |data| {
  println!("Event: {:?}", data.event);
  println!("Payload: {:?}", data.payload);
});
vm.halt();
vm.run(None); // will not be executed
