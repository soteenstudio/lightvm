use crate::types::value::Value;
pub fn format_output(val: &Value, newline: bool) {
  let out = match val {
    Value::Int32(v) => v.to_string(),
    Value::Int64(v) => v.to_string(),
    Value::Float32(v) => v.to_string(),
    Value::Float64(v) => v.to_string(),
    Value::Bool(v) => v.to_string(),
    Value::String(s) => {
      let cleaned = s.replace("::string", "");
      cleaned.trim_matches(|c| c == '\'' || c == '\"').to_string()
    }
    Value::Null => "null".to_string(),
    Value::Undefined => "undefined".to_string(),
    Value::Marker(m) => format!("<Marker: {}>", m),
  };
  if newline {
    println!("{}", out);
  } else {
    use std::io::{self, Write};
    print!("{}", out);
    let _ = io::stdout().flush();
  }
}
