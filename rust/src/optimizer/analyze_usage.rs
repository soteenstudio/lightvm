/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::{instructions::Instructions, usage::Usage};
use std::collections::HashSet;
pub fn analyze_usage(bytecode: &[Instructions]) -> Usage {
  let mut read = HashSet::new();
  let mut written = HashSet::new();
  for inst in bytecode {
    match inst {
      Instructions::Get(var_name) => {
        read.insert(var_name.clone());
      }
      Instructions::Set(var_name) | Instructions::Inc(var_name) | Instructions::Dec(var_name) => {
        written.insert(var_name.clone());
      }
      Instructions::Print | Instructions::Println => {
        read.insert("*IO*".to_string());
      }
      Instructions::Return => {
        read.insert("*RETURN*".to_string());
      }
      _ => {}
    }
  }
  Usage { read, written }
}
