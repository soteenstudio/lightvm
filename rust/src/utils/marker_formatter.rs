/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at  
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use colored::*;
use regex::Regex;
use std::sync::OnceLock;
static RE_MARKER: OnceLock<Regex> = OnceLock::new();
#[inline]
pub fn marker_formatter(text: String) -> String {
  if text.starts_with('"') && text.ends_with('"') {
    return text;
  }
  let marker_lists = ["NoInitExpression", "NoValueExpression"];
  if !marker_lists.iter().any(|&m| text.contains(m)) {
    return text;
  }
  let re = RE_MARKER.get_or_init(|| Regex::new(r"\{[a-zA-Z0-9]+\}").unwrap());
  let cleaned = re.replace_all(&text, "");
  cleaned.bright_black().to_string()
}
