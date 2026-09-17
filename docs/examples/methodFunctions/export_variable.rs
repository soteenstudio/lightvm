use lightvm::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let vm = LightVM::new(VmConfig {
    caps: vec![Capability::Debug],
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = r#"[
    ["val", "score"],
    ["push", "2"],
    ["set", "score"],
    ["export", "score"]
  ]"#;

  let optimized = tools.optimize_bytecode(raw);
  vm.load(optimized);

  let mut score_variable = vm.export("score".to_string());
  
  println!("{:?}", score_variable.call(&mut vm, vec![]));
}
