/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
#[inline(always)]
pub fn is_pure_loop(bytecode: &[Instructions], start: usize, end: usize) -> bool {
  for i in start..=end {
    if let Some(inst) = bytecode.get(i) {
      if matches!(
        inst,
        Instructions::Print
          | Instructions::Println
          | Instructions::InspectObj
          | Instructions::InspectArr
          | Instructions::Call(..)
          | Instructions::Return
          | Instructions::Stop
          | Instructions::Break(..)
          | Instructions::Jump(..)
          | Instructions::IfFalse(..)
          | Instructions::Import(..)
          | Instructions::Instantiate(..)
      ) {
        return false;
      }
    }
  }
  true
}
