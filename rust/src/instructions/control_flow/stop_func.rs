/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[inline(always)]
pub fn stop_func(call_stack: &mut Vec<usize>, ip: &mut usize) -> bool {
  if let Some(return_addr) = call_stack.pop() {
    *ip = return_addr + 1;
    return true;
  }
  false
}
