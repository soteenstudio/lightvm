use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  })
  .with_nightly(false) // To allow nightly features (default: false)
  .with_backtrace(false) // To display backtrace details in error messages (default: false)
  .with_explain(false) // To display a more detailed hint in the error message (default: false)
  .with_hint(true); // To display a hint on error messages (default: true)
  
  let tools = vm.tools();
}