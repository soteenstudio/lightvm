/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use std::io::{self, Write};
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
use smol_str::SmolStr;

#[inline(always)]
pub fn stdin_func(stack: &mut Vec<Value>) -> Result<(), VMError> {
  // 1. PastikanStdout di-flush dulu biar prompt teks sebelumnya beneran keluar ke CLI
  let _ = io::stdout().flush();

  // 2. Siapkan buffer String untuk menampung input user
  let mut input = String::new();

  // 3. Baca input dari baris CLI
  io::stdin()
    .read_line(&mut input)
    .map_err(|e| VMError::SystemError(SmolStr::new(format!("Failed to read stdin: {}", e))))?;

  // 4. Bersihin karakter newline (\n atau \r\n di Windows) di ujung teks
  let trimmed = input.trim_end_matches(['\r', '\n']);

  // 5. Masukkan hasilnya ke dalam stack sebagai Value String (Sesuaikan dengan enum Value lu, misal Value::Str atau Value::String)
  stack.push(Value::String(SmolStr::new(trimmed)));

  Ok(())
}
