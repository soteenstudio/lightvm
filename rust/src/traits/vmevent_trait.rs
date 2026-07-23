/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[cfg(not(feature = "node"))]
use crate::types::vmevent::VmEvent;
#[cfg(not(feature = "node"))]
pub trait IntoVmEvent {
  fn to_vm_event(self) -> VmEvent;
}
#[cfg(not(feature = "node"))]
impl IntoVmEvent for u32 {
  fn to_vm_event(self) -> VmEvent {
    match self {
      0 => VmEvent::Tick,
      1 => VmEvent::Halt,
      2 => VmEvent::Panic,
      _ => {
        eprintln!("Unknown event: {}", self);
        std::process::exit(1);
      }
    }
  }
}
#[cfg(not(feature = "node"))]
impl IntoVmEvent for VmEvent {
  fn to_vm_event(self) -> VmEvent {
    self
  }
}
