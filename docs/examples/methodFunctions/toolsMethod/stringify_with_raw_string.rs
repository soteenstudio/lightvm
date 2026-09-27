use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![],
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = r#"[
    ["push", 5],
    ["val", "x"],
    ["set", "x"]
  ]"#;

  let stringify = tools.stringify_ltc(raw);

  println!("{:#}", stringify.clone());
}
