let raw = r#"[
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]"#;
vm.load(tools.optimize_bytecode(raw).clone())
  .run(None);