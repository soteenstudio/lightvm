/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::itme::utils::format_duration::format_duration;
use std::hint::black_box;
use std::time::{Duration, Instant};
pub struct Benchmark {
  name: String,
  samples: usize,
  target_time: Duration,
  bytes_per_iter: Option<usize>,
}
#[allow(dead_code)]
impl Benchmark {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      samples: 15,
      target_time: Duration::from_millis(50),
      bytes_per_iter: None,
    }
  }
  pub fn samples(mut self, samples: usize) -> Self {
    assert!(samples > 0, "Benchmark samples must be greater than zero");
    self.samples = samples;
    self
  }
  pub fn target_time(mut self, target_time: Duration) -> Self {
    assert!(
      !target_time.is_zero(),
      "Benchmark target_time must be greater than zero"
    );
    self.target_time = target_time;
    self
  }
  pub fn bytes(mut self, bytes: usize) -> Self {
    self.bytes_per_iter = Some(bytes);
    self
  }
  pub fn run<F, S, T, R>(&self, mut setup: S, mut f: F)
  where
    S: FnMut() -> T,
    F: FnMut(&mut T) -> R,
  {
    let mut iterations = 1;
    loop {
      let mut state = black_box(setup());
      let start = Instant::now();
      for _ in 0..iterations {
        black_box(f(black_box(&mut state)));
      }
      let elapsed = start.elapsed();
      if elapsed >= self.target_time || iterations >= 1_000_000_000 {
        break;
      }
      if elapsed.as_millis() > 0 {
        let ratio = self.target_time.as_secs_f64() / elapsed.as_secs_f64();
        iterations = ((iterations as f64 * ratio.max(1.5)) as usize + 1).min(1_000_000_000);
      } else {
        iterations = (iterations * 10).min(1_000_000_000);
      }
    }
    for _ in 0..25 {
      let mut state = black_box(setup());
      for _ in 0..iterations {
        black_box(f(black_box(&mut state)));
      }
    }
    let mut durations = Vec::with_capacity(self.samples);
    for _ in 0..self.samples {
      let mut state = black_box(setup());
      let start = Instant::now();
      for _ in 0..iterations {
        black_box(f(black_box(&mut state)));
      }
      let elapsed = start.elapsed();
      durations.push(elapsed);
    }
    durations.sort();
    let q1_idx = durations.len() / 4;
    let q3_idx = (durations.len() * 3) / 4;
    let q1 = durations[q1_idx].as_nanos() as f64;
    let q3 = durations[q3_idx].as_nanos() as f64;
    let iqr = q3 - q1;
    let lower_bound = q1 - 1.5 * iqr;
    let upper_bound = q3 + 1.5 * iqr;
    let filtered_durations: Vec<Duration> = durations
      .iter()
      .cloned()
      .filter(|d| {
        let ns = d.as_nanos() as f64;
        ns >= lower_bound && ns <= upper_bound
      })
      .collect();
    let effective_durations = if filtered_durations.len() >= 3 {
      filtered_durations
    } else {
      durations.clone()
    };
    let min_dur = *effective_durations.first().unwrap();
    let max_dur = *effective_durations.last().unwrap();
    let median = effective_durations[effective_durations.len() / 2];
    let mean_ns = effective_durations
      .iter()
      .map(|d| d.as_nanos() as f64)
      .sum::<f64>()
      / effective_durations.len() as f64;
    let variance = effective_durations
      .iter()
      .map(|d| {
        let diff = d.as_nanos() as f64 - mean_ns;
        diff * diff
      })
      .sum::<f64>()
      / effective_durations.len() as f64;
    let std_dev_ns = variance.sqrt();
    let stability_pct = (std_dev_ns / mean_ns) * 100.0;
    let per_op = median / iterations as u32;
    let min_op = min_dur / iterations as u32;
    let max_op = max_dur / iterations as u32;
    let throughput_str = if let Some(bytes) = self.bytes_per_iter {
      let secs = per_op.as_secs_f64();
      if secs > 0.0 {
        let mib_per_sec = (bytes as f64 / (1024.0 * 1024.0)) / secs;
        format!("\n   ├─ throughput:     {:>8.2} MiB/s", mib_per_sec)
      } else {
        String::new()
      }
    } else {
      String::new()
    };
    let status_line = if stability_pct > 15.0 {
      "\n   └─ warning:        \x1b[33m[NOISY] High variance detected\x1b[0m"
    } else {
      "\n   └─ status:         \x1b[32mStable execution\x1b[0m"
    };
    println!(
      "\x1b[1m\x1b[36mBenchmark\x1b[0m[\x1b[1m\x1b[35m{:<20}\x1b[0m]\n   │\n   ├─ time per op:    {}\n   ├─ range:          [{}, {}]\n   ├─ stability:      ±{:>4.1}%{}{}",
      self.name,
      format_duration(per_op),
      format_duration(min_op),
      format_duration(max_op),
      stability_pct,
      throughput_str,
      status_line
    );
  }
}
