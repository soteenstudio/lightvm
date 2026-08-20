let raw = r#"[
  ["jump", 7],
  ["func", "add", 2, 2, 6, "a", "b"],
  ["get", "a"],
  ["get", "b"],
  ["add", "int"],
  ["println"],
  ["stop"],
  ["export", "add"]
]"#;
let optimized = tools.optimize_bytecode(raw);
vm.load(optimized);
let mut add_func = vm.export("add".to_string());
println!("{:?}", add_func(vec![5.into(), 6.into()]));