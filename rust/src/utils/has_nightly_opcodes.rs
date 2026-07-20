/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use regex::Regex;
use std::sync::OnceLock;
static NIGHTLY_OPCODES: &[&str] = &["instantiate", "import", "export"];
pub fn has_nightly_opcodes(source: &str) -> bool {
  static RE_NIGHTLY: OnceLock<Regex> = OnceLock::new();
  let regex = RE_NIGHTLY.get_or_init(|| {
    let pattern = format!(r#"(?:\[|,\s*\[)\s*"({})""#, NIGHTLY_OPCODES.join("|"));
    Regex::new(&pattern).unwrap()
  });
  regex.is_match(source)
}
