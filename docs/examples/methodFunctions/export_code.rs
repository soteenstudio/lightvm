let mut add = vm.export("add".to_string());
let args = vec![serde_json::json!(5), serde_json::json!(6)];
if let Some(result) = add_func(args) {
    println!("Result from VM: {}", result);
}
