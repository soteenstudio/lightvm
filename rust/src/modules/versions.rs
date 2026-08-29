/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::dying::colors::*;
use serde::Deserialize;
use std::fmt;
#[derive(Debug)]
struct ModuleVersions {
  carzy: String,
  gazle: String,
  itme: String,
  krates: String,
  torja: String,
  bluel: String,
  dying: String,
  vmerror: String,
}
#[derive(Debug)]
pub struct InfoVM {
  name: String,
  version: String,
  latest_version: Option<String>,
  modules: ModuleVersions,
}
#[derive(Deserialize)]
struct GitHubRelease {
  tag_name: String,
  published_at: Option<String>,
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
fn fetch_latest_github_version() -> Option<String> {
  let url = "https://api.github.com/repos/soteenstudio/lightvm/releases";
  let response = ureq::get(url)
    .set("User-Agent", "lightvm-cli")
    .call()
    .ok()?;
  let body: String = response.into_string().ok()?;
  let mut releases: Vec<(String, String)> = serde_json::from_str::<Vec<GitHubRelease>>(&body)
    .ok()?
    .into_iter()
    .filter_map(|release| {
      release
        .published_at
        .map(|published_at| (published_at, release.tag_name))
    })
    .collect();
  if releases.is_empty() {
    return None;
  }
  releases.sort_by(|a, b| b.0.cmp(&a.0));
  for (_, mut version) in releases {
    if version.contains("-proto") || version.contains(".proto") {
      continue;
    }
    if version.starts_with('v') || version.starts_with('V') {
      version.remove(0);
    }
    return Some(version);
  }
  None
}
pub fn get_versions() -> InfoVM {
  let github_latest = fetch_latest_github_version();
  InfoVM {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    latest_version: github_latest,
    modules: ModuleVersions {
      carzy: String::from("0.1.0"),
      gazle: String::from("0.1.0"),
      itme: String::from("0.1.0"),
      krates: String::from("0.1.0"),
      torja: String::from("0.1.0"),
      bluel: String::from("0.1.0"),
      dying: String::from("0.1.0"),
      vmerror: String::from("0.1.0"),
    },
  }
}
impl fmt::Display for InfoVM {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let formatted_version = humanize_version(&self.version);
    writeln!(f, "{CYAN}{BOLD}{}{RESET} v{}", self.name, formatted_version)?;
    writeln!(f, "{DARK_GRAY}modules:{RESET}")?;
    writeln!(f, "  ├─ carzy    v{}", self.modules.carzy)?;
    writeln!(f, "  ├─ gazle    v{}", self.modules.gazle)?;
    writeln!(f, "  ├─ itme     v{}", self.modules.itme)?;
    writeln!(f, "  ├─ krates   v{}", self.modules.krates)?;
    writeln!(f, "  ├─ torja    v{}", self.modules.torja)?;
    writeln!(f, "  ├─ bluel    v{}", self.modules.bluel)?;
    writeln!(f, "  ├─ dying    v{}", self.modules.dying)?;
    writeln!(f, "  └─ vmerror  v{}", self.modules.vmerror)?;
    if let Some(ref latest) = self.latest_version {
      if latest != &self.version {
        let formatted_latest = humanize_version(latest);
        writeln!(f, "\n{RESET}{DARK_GRAY}new update available:")?;
        writeln!(f, "  {RESET}{YELLOW}* {RESET}v{}", formatted_latest)?;
      }
    }
    Ok(())
  }
}
