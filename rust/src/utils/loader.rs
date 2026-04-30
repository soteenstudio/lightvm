use crate::types::instructions::Instructions;
use serde_json::Value;
pub fn parse_ltc(code: &str) -> Vec<Instructions> {
  let re = regex::Regex::new(r"\s;; IP=(\d+)").unwrap();
  let cleaned_code = re.replace_all(code, "");
  cleaned_code
    .split(';')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|line| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      let op = parts[0].to_string();
      let mut args: Vec<Value> = parts[1..]
        .iter()
        .map(|&arg| {
          if let Ok(num) = arg.parse::<f64>() {
            Value::from(num)
          } else {
            Value::from(arg)
          }
        })
        .collect();
      while args.len() < 4 {
        args.push(Value::from(""));
      }
      Instructions::from_parts(op, args)
    })
    .collect()
}
pub fn parse_ltc_to_vec(code: &str) -> Vec<Instructions> {
  let re = regex::Regex::new(r"\s;; IP=(\d+)").unwrap();
  let cleaned_code = re.replace_all(code, "");
  cleaned_code
    .split(';')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|line| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      let op = parts[0].to_string();
      let args: Vec<serde_json::Value> = parts[1..]
        .iter()
        .map(|&arg| {
          if arg.starts_with('"') && arg.ends_with('"') {
            serde_json::Value::from(&arg[1..arg.len() - 1])
          } else if let Ok(num) = arg.parse::<f64>() {
            serde_json::Value::from(num)
          } else {
            serde_json::Value::from(arg)
          }
        })
        .collect();
      Instructions::from_parts(op, args)
    })
    .collect()
}
pub fn stringify_ltc(instructions: Vec<Instructions>) -> String {
  instructions
    .iter()
    .enumerate()
    .map(|(i, instr)| {
      let parts = instr.to_parts();
      format!("{}; ;; IP={}", parts.join(" "), i)
    })
    .collect::<Vec<String>>()
    .join("\n")
}
