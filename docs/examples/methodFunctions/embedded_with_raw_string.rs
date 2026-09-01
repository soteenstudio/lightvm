use lightvm::{
  types::{capability::Capability, vmconfig::VmConfig},
  LightVM,
};

let mut vm = LightVM::new(VmConfig {
  caps: vec![Capability::Control, Capability::Observe],
  ..Default::default()
});
let raw = r#"[
  ["push", 42],
  ["stop"]
]"#;

vm.load(raw);
let result = vm.embedded();

println!("value: {}", result["value"]);
println!("outputs: {}", result["outputs"]);
println!("halted: {}", result["halted"]);
