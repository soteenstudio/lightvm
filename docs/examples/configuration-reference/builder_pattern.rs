use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, capability::Capability, time_budget::TimeBudget};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  })
  .set_max_io(100)
  .set_max_import(3)
  .set_max_alloc(50)
  .set_max_call(200)
  .set_max_jump(100)
  .set_max_ticks(1_000_000)
  .set_max_stack_size(128)
  .set_allowed_imports(vec!["math".into(), "time".into(), "utils".into()])
  .set_time_budget(TimeBudget::Cheap)
  .with_unsafe_mode(false)
  .with_nightly(false)
  .with_backtrace(false)
  .with_explain(false)
  .with_hint(true)
  .with_diagnostic_links(false);
}
