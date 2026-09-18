use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, capability::Capability, time_budget::TimeBudget};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  })
  .set_max_ticks(1_000_000)
  .set_time_budget(TimeBudget::Cheap);
  
  let tools = vm.tools();
}
