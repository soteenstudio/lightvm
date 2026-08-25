let raw = r#"[
  ["val", "score"],
  ["push", "2"],
  ["set", "score"],
  ["export", "score"]
]"#;
if let Some(optimized) = tools.optimize_bytecode_or_display(raw) {
  vm.load(optimized);
  let mut score_variable = vm.export("score".to_string());
  println!("{:?}", score_variable.call(&mut vm, vec![]));
}