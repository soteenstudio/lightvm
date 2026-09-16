/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

const FORMAT_VERSION: u8 = 1;
const MAX_RECORDS: usize = 8;
const MAX_FILE_SIZE: u64 = 32 * 1024;
const MAX_CATEGORY_LEN: usize = 48;
const MAX_CONTEXT_LEN: usize = 256;
const MAX_STATE_LEN: usize = 16;

static SEQUENCE: AtomicU64 = AtomicU64::new(1);
static STORE_LOCK: Mutex<()> = Mutex::new(());
static STORE_PATH: OnceLock<PathBuf> = OnceLock::new();

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PanicRecord {
  format_version: u8,
  sequence: u64,
  category: String,
  source: Option<SourceLocation>,
  context: String,
  state: SafeState,
  integrity: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SourceLocation {
  file: String,
  line: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SafeState {
  state: String,
  instruction_count: usize,
  function_count: usize,
  export_count: usize,
  listener_count: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PanicFile {
  format_version: u8,
  records: Vec<PanicRecord>,
}

impl SafeState {
  pub(crate) fn new(
    state: String,
    instruction_count: usize,
    function_count: usize,
    export_count: usize,
    listener_count: usize,
  ) -> Self {
    Self {
      state: bounded(&state, MAX_STATE_LEN),
      instruction_count,
      function_count,
      export_count,
      listener_count,
    }
  }
}

pub(crate) fn capture(category: &str, context: &str, state: SafeState) {
  let _ = std::panic::catch_unwind(|| {
    let mut record = PanicRecord {
      format_version: FORMAT_VERSION,
      sequence: SEQUENCE.fetch_add(1, Ordering::Relaxed),
      category: bounded(category, MAX_CATEGORY_LEN),
      source: None,
      context: bounded(context, MAX_CONTEXT_LEN),
      state,
      integrity: String::new(),
    };
    record.integrity = checksum(&record);
    let _ = append_at(store_path(), record);
  });
}

pub(crate) fn list() -> io::Result<Vec<PanicRecord>> {
  read_at(store_path())
}

pub(crate) fn export() -> io::Result<String> {
  let records = list()?;
  serde_json::to_string(&records).map_err(invalid_data)
}

pub(crate) fn clear() -> io::Result<()> {
  let _guard = STORE_LOCK
    .lock()
    .map_err(|_| io::Error::other("paniclog lock poisoned"))?;
  match fs::remove_file(store_path()) {
    Ok(()) => Ok(()),
    Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
    Err(error) => Err(error),
  }
}

fn store_path() -> &'static Path {
  STORE_PATH
    .get_or_init(|| {
      std::env::temp_dir().join(format!("lightvm-paniclog-{}.json", std::process::id()))
    })
    .as_path()
}

fn append_at(path: &Path, record: PanicRecord) -> io::Result<()> {
  let _guard = STORE_LOCK
    .lock()
    .map_err(|_| io::Error::other("paniclog lock poisoned"))?;
  let mut records = read_unlocked(path)?;
  records.push(record);
  if records.len() > MAX_RECORDS {
    records.drain(..records.len() - MAX_RECORDS);
  }
  write_atomic(
    path,
    &PanicFile {
      format_version: FORMAT_VERSION,
      records,
    },
  )
}

fn read_at(path: &Path) -> io::Result<Vec<PanicRecord>> {
  let _guard = STORE_LOCK
    .lock()
    .map_err(|_| io::Error::other("paniclog lock poisoned"))?;
  read_unlocked(path)
}

fn read_unlocked(path: &Path) -> io::Result<Vec<PanicRecord>> {
  let metadata = match fs::metadata(path) {
    Ok(metadata) => metadata,
    Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
    Err(error) => return Err(error),
  };
  if metadata.len() > MAX_FILE_SIZE {
    return Err(io::Error::new(
      io::ErrorKind::InvalidData,
      "paniclog is oversized",
    ));
  }
  let bytes = fs::read(path)?;
  let file: PanicFile = serde_json::from_slice(&bytes).map_err(invalid_data)?;
  if file.format_version != FORMAT_VERSION || file.records.len() > MAX_RECORDS {
    return Err(io::Error::new(
      io::ErrorKind::InvalidData,
      "incompatible paniclog",
    ));
  }
  for record in &file.records {
    validate(record)?;
  }
  Ok(file.records)
}

fn validate(record: &PanicRecord) -> io::Result<()> {
  let valid_lengths = record.category.len() <= MAX_CATEGORY_LEN
    && record.context.len() <= MAX_CONTEXT_LEN
    && record.state.state.len() <= MAX_STATE_LEN
    && record
      .source
      .as_ref()
      .is_none_or(|source| source.file.len() <= MAX_CONTEXT_LEN);
  if record.format_version != FORMAT_VERSION
    || !valid_lengths
    || record.integrity != checksum(record)
  {
    return Err(io::Error::new(
      io::ErrorKind::InvalidData,
      "invalid paniclog record",
    ));
  }
  Ok(())
}

fn write_atomic(path: &Path, file: &PanicFile) -> io::Result<()> {
  let bytes = serde_json::to_vec(file).map_err(invalid_data)?;
  if bytes.len() as u64 > MAX_FILE_SIZE {
    return Err(io::Error::new(
      io::ErrorKind::InvalidData,
      "paniclog is oversized",
    ));
  }
  let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
  let mut options = OpenOptions::new();
  options.write(true).create(true).truncate(true);
  #[cfg(unix)]
  {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
  }
  let mut output = options.open(&temporary)?;
  output.write_all(&bytes)?;
  output.sync_all()?;
  fs::rename(&temporary, path)
}

fn checksum(record: &PanicRecord) -> String {
  let mut copy = record.clone();
  copy.integrity.clear();
  let bytes = serde_json::to_vec(&copy).unwrap_or_default();
  let hash = bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| {
    (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
  });
  format!("{hash:016x}")
}

fn bounded(value: &str, max: usize) -> String {
  value.chars().take(max).collect()
}

fn invalid_data(error: impl std::fmt::Display) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::{SystemTime, UNIX_EPOCH};

  fn test_path(name: &str) -> PathBuf {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    std::env::temp_dir().join(format!("lightvm-{name}-{unique}.json"))
  }

  fn record(category: &str, context: &str, sequence: u64) -> PanicRecord {
    let mut record = PanicRecord {
      format_version: FORMAT_VERSION,
      sequence,
      category: bounded(category, MAX_CATEGORY_LEN),
      source: None,
      context: bounded(context, MAX_CONTEXT_LEN),
      state: SafeState::new("Running".into(), 2, 1, 0, 1),
      integrity: String::new(),
    };
    record.integrity = checksum(&record);
    record
  }

  #[test]
  fn record_is_bounded_and_contains_no_panic_payload() {
    let secret = "token=very-secret-value";
    let record = record(&"x".repeat(100), &"y".repeat(1000), 1);
    let serialized = serde_json::to_string(&record).unwrap();
    assert_eq!(record.category.len(), MAX_CATEGORY_LEN);
    assert_eq!(record.context.len(), MAX_CONTEXT_LEN);
    assert!(!serialized.contains(secret));
  }

  #[test]
  fn retention_keeps_only_newest_records() {
    let path = test_path("retention");
    for sequence in 0..MAX_RECORDS as u64 + 2 {
      append_at(&path, record("listener_panic", "compile_start", sequence)).unwrap();
    }
    let records = read_at(&path).unwrap();
    assert_eq!(records.len(), MAX_RECORDS);
    assert_eq!(records[0].sequence, 2);
    let _ = fs::remove_file(path);
  }

  #[test]
  fn rejects_corruption_and_incompatible_versions() {
    let corrupt = test_path("corrupt");
    fs::write(&corrupt, b"{incomplete").unwrap();
    assert_eq!(
      read_at(&corrupt).unwrap_err().kind(),
      io::ErrorKind::InvalidData
    );
    let incompatible = test_path("version");
    fs::write(&incompatible, br#"{"format_version":2,"records":[]}"#).unwrap();
    assert_eq!(
      read_at(&incompatible).unwrap_err().kind(),
      io::ErrorKind::InvalidData
    );
    let _ = fs::remove_file(corrupt);
    let _ = fs::remove_file(incompatible);
  }

  #[test]
  fn rejects_modified_record() {
    let path = test_path("integrity");
    append_at(&path, record("listener_panic", "compile_start", 1)).unwrap();
    let mut bytes = fs::read_to_string(&path).unwrap();
    bytes = bytes.replace("compile_start", "compile_other");
    fs::write(&path, bytes).unwrap();
    assert_eq!(
      read_at(&path).unwrap_err().kind(),
      io::ErrorKind::InvalidData
    );
    let _ = fs::remove_file(path);
  }
}
