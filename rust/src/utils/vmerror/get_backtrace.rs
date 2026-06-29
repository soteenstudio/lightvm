/*
 * Copyright 2026 SoTeen Studio
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
  static CAPTURED_BACKTRACE: RefCell<Option<Backtrace>> = RefCell::new(None);
}

pub fn capture_backtrace() {
  CAPTURED_BACKTRACE.with(|bt| {
    // Only capture if not already captured (first error in this execution)
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
      let bt_str = format!("{:?}", backtrace);
      let target_crate = "lightvm";
      bt_str
        .lines()
        .filter(|line| line.contains(target_crate) && !line.contains("get_backtrace") && !line.contains("capture_backtrace"))
        .enumerate()
        .take(5)
        .map(|(i, line)| {
          let parts: Vec<&str> = line.split_whitespace().collect();
          let func_name = parts.last().unwrap_or(&"unknown");
          format!(" {RESET}{CYAN}│   {DARK_GRAY}{}: {}", i + 1, func_name)
        })
        .collect::<Vec<_>>()
        .join("\n")
    } else {
      String::new()
    }
  })
}
