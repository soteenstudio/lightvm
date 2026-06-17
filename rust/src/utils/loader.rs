/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use regex::Regex;
use serde_json::Value;
use std::fmt::Write;
use std::sync::OnceLock;
static RE_IP: OnceLock<Regex> = OnceLock::new();
fn get_re() -> &'static Regex {
  RE_IP.get_or_init(|| Regex::new(r"\s;; IP=(\d+)").unwrap())
}
pub fn parse_ltc(code: &str) -> Vec<Instructions> {
  let cleaned_code = get_re().replace_all(code, "");
  cleaned_code
    .as_ref()
    .split(';')
    .map(|s: &str| s.trim())
    .filter(|s: &&str| !s.is_empty())
    .map(|line: &str| {
      let mut parts = line.split_whitespace();
      let op = parts.next().unwrap_or("").to_string();
      let mut args: Vec<Value> = parts
        .map(|arg: &str| {
          if arg.starts_with('"') && arg.ends_with('"') && arg.len() >= 2 {
            Value::from(&arg[1..arg.len() - 1])
          } else if let Ok(num) = arg.parse::<f64>() {
            Value::from(num)
          } else {
            Value::from(arg)
          }
        })
        .collect();
      if args.len() < 4 {
        args.resize(4, Value::from(""));
      }
      Instructions::from_parts(op, args)
    })
    .collect()
}
pub fn parse_ltc_to_vec(code: &str) -> Vec<Instructions> {
  let cleaned_code = get_re().replace_all(code, "");
  cleaned_code
    .as_ref()
    .split(';')
    .map(|s: &str| s.trim())
    .filter(|s: &&str| !s.is_empty())
    .map(|line: &str| {
      let mut parts = line.split_whitespace();
      let op = parts.next().unwrap_or("").to_string();
      let args: Vec<serde_json::Value> = parts
        .map(|arg: &str| {
          if arg.starts_with('"') && arg.ends_with('"') && arg.len() >= 2 {
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
  let mut result = String::with_capacity(instructions.len() * 40);
  for (i, instr) in instructions.iter().enumerate() {
    if i > 0 {
      result.push('\n');
    }
    let parts = instr.to_parts();
    for (p_idx, part) in parts.iter().enumerate() {
      if p_idx > 0 {
        result.push(' ');
      }
      result.push_str(part);
    }
    let _ = write!(result, "; ;; IP={}", i);
  }
  result
}
