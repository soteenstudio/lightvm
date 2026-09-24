use lightvm::LightVM;
use lightvm::types::{capability::Capability, vmconfig::VmConfig};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });
  let tools = vm.tools();

  let str_val = r#"
    push 5; ;; IP=0
    val x; ;; IP=1
    set x; ;; IP=2
  "#;

  let parsed = tools.parse_ltc_array(str_val);

  println!("{:#}", parsed.clone());
}
