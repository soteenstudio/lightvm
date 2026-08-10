use lightvm::types::{
  compile_config::CompileConfig,
  target_arch::TargetArch,
  file_type::FileType
};

let raw = r#"[
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]"#;
let optimized = tools.optimize_bytecode(raw);
vm.load(optimized.clone())
  .compile(CompileConfig {
    target_arch: TargetArch::AArch64,
    file_type: FileType::Binary,
    path: "./bin/output",
  });