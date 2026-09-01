use lightvm::{
  types::{capability::Capability, vmconfig::VmConfig},
  LightVM,
};

let mut vm = LightVM::new(VmConfig {
  caps: vec![Capability::Control, Capability::Observe],
  ..Default::default()
});
let program = serde_json::json!([
  ["push", 42],
  ["stop"]
]);

vm.load(program);
let result = vm.embedded();

println!("value: {}", result["value"]);
println!("outputs: {}", result["outputs"]);
println!("halted: {}", result["halted"]);
