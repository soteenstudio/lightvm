use lightvm::LightVM;
use lightvm::types::{
  vmconfig::VmConfig,
  runtime_config::RuntimeConfig,
  error_options::ErrorOptions,
  capability::Capability
};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    runtime_config: RuntimeConfig {
      nightly: false // To allow nightly features (default: false)
    },
    error_options: ErrorOptions {
      backtrace: false, // To display backtrace details in error messages (default: false)
      explain: false, // To display a more detailed hint in the error message (default: false)
      hint: true // To display a hint on error messages (default: true)
    }
  });
  
  let tools = vm.tools();
}