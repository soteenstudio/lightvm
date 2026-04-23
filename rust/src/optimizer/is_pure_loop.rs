/* * Copyright 2026 Clay
 * * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * * http:
 */
use crate::types::instructions::Instructions;
pub fn is_pure_loop(bytecode: &[Instructions], start: usize, end: usize) -> bool {
  for i in start..=end {
    if let Some(inst) = bytecode.get(i) {
      if matches!(
        inst,
        Instructions::Print
          | Instructions::Println
          | Instructions::Call(..)
          | Instructions::Return
          | Instructions::Stop
      ) {
        return false;
      }
    }
  }
  true
}
