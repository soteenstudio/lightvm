/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::codegen::arch::aarch64_isel::{
  io_isel::io_isel, math_isel::math_isel, stack_isel::stack_isel,
};
use crate::modules::carzy::arch::aarch64::AArch64Builder;
use crate::modules::gazle::specialized_instructions::specialized_instructions;
use crate::modules::torja::resolve_symbols::resolve_symbols;
use crate::types::instructions::Instructions;
use crate::utils::vmerror::VMError;
use ahash::AHashMap;
use smol_str::SmolStr;
pub fn compile_aarch64(mut instructions: Vec<Instructions>) -> Result<String, VMError> {
  specialized_instructions(&mut instructions);
  let empty_imports: AHashMap<SmolStr, crate::types::value::Value> = AHashMap::new();
  let (var_count, _symbol_table) = resolve_symbols(&mut instructions, &empty_imports);
  let mut builder = AArch64Builder::new()
    .global("main")
    .symbol_type("main", "function")
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
  let total_stack_slots = var_count;
  if total_stack_slots > 0 {
    let stack_bytes = total_stack_slots * 16;
    builder = builder
      .comment(&format!("Allocate local frame: {} vars", var_count))
      .sub("sp", "sp", &format!("#{}", stack_bytes));
  }
  for (index, inst) in instructions.iter().enumerate() {
    let _label_prefix = format!("const_{}", index);
    builder = match inst {
      Instructions::PushInt16(_)
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
      Instructions::Add(_)
      | Instructions::Sub(_)
      | Instructions::Mul(_)
      | Instructions::Div(_)
      | Instructions::Mod(_) => math_isel(builder, inst),
      _ => {
        return Err(VMError::InvalidOpcode {
          ip: index,
          code: "UNKNOWN_OPCODE".into(),
        });
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
#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::value::Value;
  #[test]
  fn specializes_raw_cli_push_int16_before_selection() {
    let asm = compile_aarch64(vec![
      Instructions::Push(Value::Int16(128)),
      Instructions::Println,
    ])
    .expect("raw CLI Push(Int16) program should compile");
    assert!(asm.contains("PushInt16(128)"));
    assert!(!asm.contains("Push (Generic)"));
    assert!(asm.contains("mov x10, #0"));
    assert!(asm.contains("str x10, [sp]"));
    assert!(asm.contains("str x9, [sp, #8]"));
  }
  #[test]
  fn specializes_raw_cli_push_bool_before_selection() {
    let asm = compile_aarch64(vec![
      Instructions::Push(Value::Bool(true)),
      Instructions::Println,
    ])
    .expect("raw CLI Push(Bool) program should compile");
    assert!(asm.contains("PushBool(true)"));
    assert!(!asm.contains("Push (Generic)"));
  }
  fn compile_bool_through_all_outputs(b: bool) -> String {
    compile_aarch64(vec![
      Instructions::PushBool(b),
      Instructions::Print,
      Instructions::PushBool(b),
      Instructions::Println,
      Instructions::PushBool(b),
      Instructions::Stdout,
      Instructions::PushBool(b),
      Instructions::Stdoutln,
    ])
    .expect("boolean CLI output program should compile")
  }
  #[test]
  fn compiles_true_through_print_println_stdout_stdoutln() {
    let asm = compile_bool_through_all_outputs(true);
    assert!(asm.contains("str x9, [sp, #8]"));
    assert!(asm.contains("bl lightvm_print"));
    assert!(asm.contains("bl lightvm_println"));
    assert!(asm.contains("bl lightvm_stdout"));
    assert!(asm.contains("bl lightvm_stdoutln"));
    assert!(!asm.contains("ldr x0, [sp]"));
  }
  #[test]
  fn compiles_false_through_print_println_stdout_stdoutln() {
    let asm = compile_bool_through_all_outputs(false);
    assert!(asm.contains("mov x9, #0"));
    assert!(asm.contains("str x9, [sp, #8]"));
    assert!(asm.contains("bl lightvm_print"));
    assert!(asm.contains("bl lightvm_println"));
    assert!(asm.contains("bl lightvm_stdout"));
    assert!(asm.contains("bl lightvm_stdoutln"));
    assert!(!asm.contains("ldr x0, [sp]"));
  }
}
