/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::vmevent::VmEvent;
pub trait IntoVmEvent {
  fn to_vm_event(self) -> VmEvent;
}
impl IntoVmEvent for &str {
  fn to_vm_event(self) -> VmEvent {
    match self {
      "tick" => VmEvent::Tick,
      "halt" => VmEvent::Halt,
      "panic" => VmEvent::Panic,
      _ => {
        eprintln!("Unknown event: {}", self);
        std::process::exit(1);
      }
    }
  }
}
impl IntoVmEvent for VmEvent {
  fn to_vm_event(self) -> VmEvent {
    self
  }
}
