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
      nightly: false // Allow nightly features (default: false)
    }),
    error_options: Some(ErrorOptions {
      backtrace: false, // Display backtrace details in error messages (default: false)
      explain: false, // Display a more detailed hint in the error message (default: false)
      hint: true // Display a hint on error messages (default: true)
    }),
    security_config: Some(SecurityConfig {
      max_io: 100, // Maximum number of I/O operations allowed (default: 100)
      max_import: 3, // Maximum number of allowed module imports (default: 3)
      max_alloc: 50, // Maximum number of memory allocations allowed (default: 50)
      max_call: 200, // Maximum number of nested function calls allowed (default: 200)
      max_jump: 100, // Maximum number of control flow jumps allowed (default: 100)
      max_ticks: 1_000_000, // Maximum number of execution ticks before stopping (default: 1,000,000)
      max_stack_size: 128, // Maximum number of items the stack can hold (default: 128)
      allowed_imports: vec!["math".into(), "time".into(), "utils".into()], // Whitelist of modules that can be imported
      time_budget: TimeBudget::Cheap, // Sets the execution time budget limit to prevent infinite loops (Default: Cheap)
      unsafe_mode: false // Enable or disable system-level unsafe operations (default: false)
    })
  });
  
  let tools = vm.tools();
}