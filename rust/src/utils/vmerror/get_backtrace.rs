/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::utils::vmerror::colors::*;
use backtrace::Backtrace;
use std::cell::RefCell;
thread_local! {
  static CAPTURED_BACKTRACE: RefCell<Option<Backtrace>> = const { RefCell::new(None) };
}
pub fn capture_backtrace() {
  CAPTURED_BACKTRACE.with(|bt| {
    if bt.borrow().is_none() {
      *bt.borrow_mut() = Some(Backtrace::new());
    }
  });
}
pub fn clear_backtrace() {
  CAPTURED_BACKTRACE.with(|bt| {
    *bt.borrow_mut() = None;
  });
}
pub fn get_backtrace() -> String {
  CAPTURED_BACKTRACE.with(|bt| {
    let bt_opt = bt.borrow();
    if let Some(backtrace) = bt_opt.as_ref() {
      let mut trace = String::new();
      let mut count = 0;
      let mut resolved_bt = backtrace.clone();
      resolved_bt.resolve();
      for frame in resolved_bt.frames() {
        for symbol in frame.symbols() {
          if let Some(name) = symbol.name() {
            let name_str = name.as_str().unwrap_or("unknown");
            if name_str.contains("lightvm")
               && !name_str.contains("get_backtrace")
               && !name_str.contains("capture_backtrace")
            {
              count += 1;
              if count <= 5 {
                trace.push_str(&format!(" {RESET}{CYAN}│   {DARK_GRAY}{}: {}\n", count, name_str));
              }
            }
          }
        }
      }
      if count == 0 {
        format!(" {RESET}{CYAN}│   {DARK_GRAY}1: vmerror::backtrace_unavailable\n {RESET}{CYAN}│   {DARK_GRAY}2: vmerror::no_symbol_found_or_stack_truncated{RESET}")
      } else {
        trace.trim_end().to_string()
      }
    } else {
      format!(" {RESET}{CYAN}│   {DARK_GRAY}1: vmerror::backtrace_unavailable\n {RESET}{CYAN}│   {DARK_GRAY}2: vmerror::empty_stack_snapshot{RESET}")
    }
  })
}
