/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::codegen::arch::aarch64_isel::{io_isel::io_isel, stack_isel::stack_isel};
use crate::modules::carzy::arch::aarch64::AArch64Builder;
use crate::modules::torja::resolve_symbols::resolve_symbols;
use crate::types::instructions::Instructions;
use ahash::AHashMap;
use smol_str::SmolStr;
pub fn compile_aarch64(mut instructions: Vec<Instructions>) -> Result<String, String> {
  let empty_imports: AHashMap<SmolStr, crate::types::value::Value> = AHashMap::new();
  let (var_count, _symbol_table) = resolve_symbols(&mut instructions, &empty_imports);
  let init_stack_capacity = instructions
    .iter()
    .find_map(|inst| {
      if let Instructions::InitStack(size) = inst {
        Some(*size as usize)
      } else {
        None
      }
    })
    .unwrap_or(0);
  let mut builder = AArch64Builder::new()
    .global("main")
    .rodata()
    .inject_io_constants()
    .text()
    .label("main");
  builder = builder
    .comment("Establish stack frame")
    .sub("sp", "sp", "#16")
    .str("x19", "sp")
    .mov("x19", "sp")
    .add("x19", "x19", "#16");
  let total_stack_slots = var_count + init_stack_capacity;
  if total_stack_slots > 0 {
    let stack_bytes = total_stack_slots * 16;
    builder = builder
      .comment(&format!(
        "Allocate local frame: {} vars + {} InitStack slots",
        var_count, init_stack_capacity
      ))
      .sub("sp", "sp", &format!("#{}", stack_bytes));
  }
  for (index, inst) in instructions.iter().enumerate() {
    let _label_prefix = format!("const_{}", index);
    builder = match inst {
      Instructions::InitStack(_)
      | Instructions::PushInt16(_)
      | Instructions::PushInt32(_)
      | Instructions::PushInt64(_)
      | Instructions::PushInt128(_)
      | Instructions::PushFloat16(_)
      | Instructions::PushFloat32(_)
      | Instructions::PushFloat64(_)
      | Instructions::PushString(_)
      | Instructions::PushArray(_)
      | Instructions::PushObject(_)
      | Instructions::PushBool(_)
      | Instructions::PushUndefined
      | Instructions::PushNull
      | Instructions::PushNaN
      | Instructions::Push(_)
      | Instructions::ValIdx(_)
      | Instructions::SetIdx(_)
      | Instructions::GetIdx(_)
      | Instructions::Dup
      | Instructions::Swap => stack_isel(builder, inst),
      Instructions::Print
      | Instructions::Println
      | Instructions::Stdout
      | Instructions::Stdoutln
      | Instructions::Stdin
      | Instructions::InspectObj
      | Instructions::InspectArr
      | Instructions::ClearScreen => io_isel(builder, inst),
      _ => {
        return Err(format!(
          "Unsupported instruction at index {}: {:?}",
          index, inst
        ));
      }
    };
  }
  Ok(
    builder
      .comment("Restore stack pointer from frame base")
      .inst("ldr", "x9, [x19, #-16]")
      .mov("sp", "x19")
      .mov("x19", "x9")
      .ret()
      .build(),
  )
}
