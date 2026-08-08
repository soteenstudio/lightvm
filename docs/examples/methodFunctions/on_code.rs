use lightvm::vmevent::VmEvent;

vm.on(VmEvent.Halt, |payload| {
  println!("Halted: {:?}", payload);
});
