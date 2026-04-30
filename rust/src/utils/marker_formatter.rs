/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use colored::*;
use regex::Regex;
pub fn marker_formatter(text: String) -> String {
  let marker_lists = ["NoInitExpression", "NoValueExpression"];
  let contains_marker = marker_lists.iter().any(|&m| text.contains(m));
  let is_not_quoted = !(text.starts_with('"') && text.ends_with('"'));
  if contains_marker && is_not_quoted {
    let re = Regex::new(r"\{[a-zA-Z0-9]+\}").unwrap();
    let cleaned_text = re.replace_all(&text, "").to_string();
    return cleaned_text.bright_black().to_string();
  }
  text
}
