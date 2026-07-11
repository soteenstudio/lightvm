vm.on("halt", |payload| {
  println!("Halted: ", payload);
});
