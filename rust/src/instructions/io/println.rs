use crate::types::value::Value;
use crate::utils::format_output::format_output; // <--- Import helpernya

pub fn println_func(val: Value) {
  format_output(&val, true);
}
