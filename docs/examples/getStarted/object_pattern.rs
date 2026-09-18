use lightvm::LightVM;
use lightvm::types::{
  vmconfig::VmConfig,
  security_config::SecurityConfig,
  capability::Capability,
  time_budget::TimeBudget
};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    security_config: Some(SecurityConfig {
      max_ticks: 1_000_000,
      time_budget: TimeBudget::Cheap,
      ..Defualt::default(),
    }),
    ..Defualt::defualt(),
  });
  
  let tools = vm.tools();
}
