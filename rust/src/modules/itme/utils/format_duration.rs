/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::time::Duration;
pub fn format_duration(d: Duration) -> String {
  let ns = d.as_nanos();
  if ns < 1_000 {
    format!("{} ns", ns)
  } else if ns < 1_000_000 {
    format!("{:.2} µs", ns as f64 / 1_000.0)
  } else if ns < 1_000_000_000 {
    format!("{:.2} ms", ns as f64 / 1_000_000.0)
  } else {
    format!("{:.2} s", d.as_secs_f64())
  }
}
