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
pub fn stop_func(stack: &mut Vec<Value>, call_stack: &mut Vec<usize>, ip: &mut usize) -> bool {
  if stack.len() > 50 {
    stack.truncate(50);
  }
  if let Some(return_addr) = call_stack.pop() {
    *ip = return_addr + 1;
    return true;
  }
  false
}
