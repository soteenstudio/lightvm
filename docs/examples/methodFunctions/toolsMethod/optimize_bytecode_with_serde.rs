let raw = serde_json::json!([
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]);
let optimized = tools.optimize_bytecode(raw);
println!(optimized.clone());
