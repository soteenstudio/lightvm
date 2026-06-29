/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::vmerror::colors::*;
use backtrace::Backtrace;
pub fn get_backtrace() -> String {
  let bt = Backtrace::new();
  let bt_str = format!("{:?}", bt);
  let target_crate = "lightvm";
  bt_str
    .lines()
    .filter(|line| line.contains(target_crate) && !line.contains("get_backtrace"))
    .enumerate()
    .take(5)
    .map(|(i, line)| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      let func_name = parts.last().unwrap_or(&"unknown");
      format!(" {RESET}{CYAN}│   {DARK_GRAY}{}: {}", i + 1, func_name)
    })
    .collect::<Vec<_>>()
    .join("\n")
}
