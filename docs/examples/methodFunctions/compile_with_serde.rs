use lightvm::LightVM;
use lightvm::types::{
  vmconfig::VmConfig,
  capability::Capability,
  compile_config::CompileConfig,
  target_arch::TargetArch,
  file_type::FileType
};

fn main() {
  let vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control],
    ..Default::default()
  });
  let tools = vm.tools();

  let raw = serde_json::json!([
    ["push", 5],
    ["val", "x"],
    ["set", "x"]
  ]);

  let optimized = tools.optimize_bytecode(raw);
  vm.load(optimized.clone());

  vm.compile(CompileConfig {
    target_arch: TargetArch::AArch64,
    file_type: FileType::Binary,
    path: "./bin/output",
  });
}
