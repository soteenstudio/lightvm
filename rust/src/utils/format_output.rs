/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use std::io::{self, BufWriter, Write as IoWrite};
use std::sync::OnceLock;
use std::fmt::Write;
static STDOUT_BUFFER: OnceLock<std::sync::Mutex<BufWriter<io::Stdout>>> = OnceLock::new();
#[inline]
pub fn format_output(val: &Value, newline: bool) {
  let mut writer = STDOUT_BUFFER
    .get_or_init(|| std::sync::Mutex::new(BufWriter::with_capacity(16 * 1024, io::stdout())))
    .lock()
    .unwrap();

  // 1. Buat FilteredWriter untuk memproses nilai
  let mut f_writer = crate::utils::filtered_writer::FilteredWriter {
    buffer: String::with_capacity(64), // Alokasi awal kecil aja
    state: 0,
  };

  // 2. Tulis value ke f_writer dulu (biar difilter)
  let _ = write!(f_writer, "{}", val);

  // 3. Pastikan karakter yang nanggung di-flush
  if f_writer.state > 0 {
    f_writer.flush_failed_match();
  }

  // 4. Baru tulis hasil yang udah bersih ke STDOUT yang asli
  let _ = write!(writer, "{}", f_writer.buffer);

  if newline {
    let _ = writeln!(writer);
    let _ = writer.flush();
  }
}
