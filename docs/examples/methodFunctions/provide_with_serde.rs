vm.provide(serde_json::json!({
  "name": "John Doe",
  "force": 2021
}));
let raw = serde_json::json!([
  ["get", "name"],
  ["println"],
  ["get", "force"],
  ["println"]
]);
