let raw = r#"[
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]"#;
let optimized = tools.optimize_bytecode(raw)?;
vm.load(optimized.clone())
  .run(None);
