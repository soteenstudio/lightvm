/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use serde_json::Value;
pub fn parse_ltc(code: String) -> Vec<Instructions> {
  let re = regex::Regex::new(r"\s;; IP=(\d+)").unwrap();
  let cleaned_code = re.replace_all(&code, "");
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
