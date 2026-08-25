let raw = serde_json::json!([
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]);
if let Some(optimized) = tools.optimize_bytecode_or_display(raw) {
  vm.load(optimized).run(None);
}