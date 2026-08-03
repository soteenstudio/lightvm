use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  })
  .set_max_io(100) // Maximum number of I/O operations allowed (default: 100)
  .set_max_import(3) // Maximum number of allowed module imports (default: 3)
  .set_max_alloc(50) // Maximum number of memory allocations allowed (default: 50)
  .set_max_call(200) // Maximum number of nested function calls allowed (default: 200)
  .set_max_jump(100) // Maximum number of control flow jumps allowed (default: 100)
  .set_max_ticks(1_000_000) // Maximum number of execution ticks before stopping (default: 1,000,000)
  .set_max_stack_size(128) // Maximum number of items the stack can hold (default: 128)
  .set_allowed_imports(vec!["math".into(), "time".into(), "utils".into()]) // Whitelist of modules that can be imported
  .with_unsafe_mode(false) // Enable or disable system-level unsafe operations (default: false)
  .with_nightly(false) // Allow nightly features (default: false)
  .with_backtrace(false) // Display backtrace details in error messages (default: false)
  .with_explain(false) // Display a more detailed hint in the error message (default: false)
  .with_hint(true); // Display a hint on error messages (default: true)
  
  let tools = vm.tools();
}