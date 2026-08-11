/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::hint::black_box;
use std::time::{Duration, Instant};
fn format_duration(d: Duration) -> String {
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
      samples: 10,
      target_time: Duration::from_millis(50),
      bytes_per_iter: None,
    }
  }
  pub fn bytes(mut self, bytes: usize) -> Self {
    self.bytes_per_iter = Some(bytes);
    self
  }
  /// Run the benchmark.
  ///
  /// The benchmark callback `f` must return the calculated value to prevent
  /// the operation from being optimized away by the compiler.
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
    for _ in 0..10 {
      let mut state = black_box(setup());
      black_box(f(black_box(&mut state)));
    }
    let mut durations = Vec::with_capacity(self.samples);
    for _ in 0..self.samples {
      let mut state = black_box(setup());
      let start = Instant::now();
      for _ in 0..iterations {
        black_box(f(black_box(&mut state)));
      }
      durations.push(start.elapsed());
    }
    durations.sort();
    let median = durations[durations.len() / 2];
    let per_op = median / iterations as u32;
    print!(
      "[BENCH] {:<20} | {:>10} per op ({} iters)",
      self.name,
      format_duration(per_op),
      iterations
    );
    if let Some(bytes) = self.bytes_per_iter {
      let secs = per_op.as_secs_f64();
      if secs > 0.0 {
        let mib_per_sec = (bytes as f64 / (1024.0 * 1024.0)) / secs;
        print!(" | {:>8.2} MiB/s", mib_per_sec);
      }
    }
    println!();
  }
}
