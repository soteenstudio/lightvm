use lightvm::LightVM;
use lightvm::types::{
  vmconfig::VmConfig,
  runtime_config::RuntimeConfig,
  error_options::ErrorOptions,
  security_config::SecurityConfig,
  capability::Capability,
  time_budget::TimeBudget
};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    runtime_config: Some(RuntimeConfig {
      nightly: false
    }),
    error_options: Some(ErrorOptions {
      backtrace: false,
      explain: false,
      hint: true,
      diagnostic_links: false
    }),
    security_config: Some(SecurityConfig {
      max_io: 100,
      max_import: 3,
      max_alloc: 50,
      max_call: 200,
      max_jump: 100,
      max_ticks: 1_000_000,
      max_stack_size: 128,
      allowed_imports: vec!["math".into(), "time".into(), "utils".into()],
      time_budget: TimeBudget::Cheap,
      unsafe_mode: false
    })
  });
}
