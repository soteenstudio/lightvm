use crate::types::value::Value;
use crate::utils::format_output::format_output;
pub fn print_func(val: Value) {
  format_output(&val, false);
}
