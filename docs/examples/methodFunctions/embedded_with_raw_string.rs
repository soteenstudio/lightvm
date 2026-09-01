let raw = r#"[
  ["push", 42],
  ["stop"]
]"#;
let optimized = tools.optimize_bytecode(raw);
vm.load(optimized.clone())
let result = vm.embedded();
println!("value: {}", result["value"]);
println!("outputs: {}", result["outputs"]);
println!("halted: {}", result["halted"]);
