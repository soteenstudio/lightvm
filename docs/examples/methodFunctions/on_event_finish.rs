use lightvm::vmevent::VmEvent;

vm.on(VmEvent::Finish, |data| {
  println!("Event: {:?}", data.event);
  println!("Payload: {:?}", data.payload);
});
vm.run();
