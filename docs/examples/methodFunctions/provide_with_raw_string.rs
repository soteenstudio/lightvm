vm.provide(serde_json::json!({
  "name": "John Doe",
  "force": 2021
}));
let raw = r#"[
  ["get", "name"],
  ["println"],
  ["get", "force"],
  ["println"]
]"#;
