/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::carzy::arch::aarch64::AArch64Builder;
use crate::types::instructions::Instructions;
pub fn io_isel(builder: AArch64Builder, inst: &Instructions) -> AArch64Builder {
  match inst {
    Instructions::Print => builder
      .comment("Print (Runtime Mock)")
      .ldr("x0", "sp")
      .inst("bl", "lightvm_print")
      .add("sp", "sp", "#16"),
    Instructions::Println => builder
      .comment("Println (Runtime Mock)")
      .ldr("x0", "sp")
      .inst("bl", "lightvm_println")
      .add("sp", "sp", "#16"),
    Instructions::Stdout => builder
      .comment("Stdout (Runtime Mock)")
      .ldr("x0", "sp")
      .inst("bl", "lightvm_stdout")
      .add("sp", "sp", "#16"),
    Instructions::Stdoutln => builder
      .comment("Stdoutln (Runtime Mock)")
      .ldr("x0", "sp")
      .inst("bl", "lightvm_stdoutln")
      .add("sp", "sp", "#16"),
    Instructions::Stdin => builder
      .comment("Stdin (Runtime Mock)")
      .sub("sp", "sp", "#16")
      .add("x0", "sp", "#0")
      .inst("bl", "lightvm_stdin"),
    Instructions::InspectObj => builder
      .comment("InspectObj (Runtime Mock)")
      .ldr("x0", "sp")
      .inst("bl", "lightvm_inspect_obj"),
    Instructions::InspectArr => builder
      .comment("InspectArr (Runtime Mock)")
      .ldr("x0", "sp")
      .inst("bl", "lightvm_inspect_arr"),
    Instructions::ClearScreen => builder
      .comment("ClearScreen (Runtime Mock)")
      .inst("bl", "lightvm_clear_screen"),
    _ => builder,
  }
}
