/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::fmt;
pub const _YELLOW: &str = "\x1b[33m";
pub const CYAN: &str = "\x1b[36m";
pub const DARK_GRAY: &str = "\x1b[2;37m";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
#[derive(Debug)]
struct ModuleVersions {
  carzy: String,
  gazle: String,
  itme: String,
  krates: String,
  torja: String,
}
#[derive(Debug)]
pub struct InfoVM {
  name: String,
  version: String,
  modules: ModuleVersions,
}
fn humanize_version(raw_version: &str) -> String {
  if let Some(nightly_idx) = raw_version.find("-nightly.") {
    let base_ver = &raw_version[..nightly_idx];
    let remainder = &raw_version[nightly_idx + 9..];
    let parts: Vec<&str> = remainder.split('.').collect();
    if parts.len() >= 2 {
      let date_str = parts[0];
      let hash_str = parts[1];
      if date_str.len() == 8 {
        let year = &date_str[0..4];
        let month = &date_str[4..6];
        let day = &date_str[6..8];
        let month_name = match month {
          "01" => "Jan",
          "02" => "Feb",
          "03" => "Mar",
          "04" => "Apr",
          "05" => "May",
          "06" => "Jun",
          "07" => "Jul",
          "08" => "Aug",
          "09" => "Sep",
          "10" => "Oct",
          "11" => "Nov",
          "12" => "Dec",
          _ => month,
        };
        return format!(
          "{} (Nightly {} {} {}, {})",
          base_ver, day, month_name, year, hash_str
        );
      }
    }
  }
  raw_version.to_string()
}
#[cfg(test)]
mod tests {
  use super::humanize_version;

  #[test]
  fn humanize_version_returns_bare_nightly_version_unchanged() {
    assert_eq!(humanize_version("0.1.0-nightly"), "0.1.0-nightly");
  }

  #[test]
  fn humanize_version_formats_valid_nightly_version() {
    assert_eq!(
      humanize_version("0.1.0-nightly.20260828.d36cc1e"),
      "0.1.0 (Nightly 28 Aug 2026, d36cc1e)"
    );
  }
}
pub fn get_versions() -> InfoVM {
  InfoVM {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    modules: ModuleVersions {
      carzy: String::from("0.1.0"),
      gazle: String::from("0.1.0"),
      itme: String::from("0.1.0"),
      krates: String::from("0.1.0"),
      torja: String::from("0.1.0"),
    },
  }
}
impl fmt::Display for InfoVM {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let formatted_version = humanize_version(&self.version);
    writeln!(f, "{CYAN}{BOLD}{}{RESET} v{}", self.name, formatted_version)?;
    writeln!(f, "{DARK_GRAY}modules:{RESET}")?;
    writeln!(f, "  ├─ carzy  v{}", self.modules.carzy)?;
    writeln!(f, "  ├─ gazle  v{}", self.modules.gazle)?;
    writeln!(f, "  ├─ itme   v{}", self.modules.itme)?;
    writeln!(f, "  ├─ krates v{}", self.modules.krates)?;
    write!(f, "  └─ torja  v{}", self.modules.torja)
  }
}
