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
use crate::types::primitive_types::PrimitiveTypes;
pub fn math_isel(builder: AArch64Builder, inst: &Instructions) -> AArch64Builder {
  match inst {
    Instructions::Add(num_type) => {
      let type_tag = match num_type {
        PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct => 0,
        PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl => 2,
        PrimitiveTypes::Str => 3,
      };
      let comment_str = format!("Add({:?})", num_type);
      let mut b = builder.comment(&comment_str);
      b = b.ldr("x1", "sp, #8");
      b = b.ldr("x2", "sp, #24");
      match num_type {
        PrimitiveTypes::Hlf | PrimitiveTypes::Flt => {
          b = b.inst("fadd", "s0, s2, s1");
          b = b.inst("str", "s0, [sp, #24]");
        }
        PrimitiveTypes::Dbl => {
          b = b.inst("fadd", "d0, d2, d1");
          b = b.inst("str", "d0, [sp, #24]");
        }
        PrimitiveTypes::Oct => {
          b = b.inst("adds", "x9, x2, x1");
          b = b.str("x9", "sp, #24");
        }
        _ => {
          b = b.add("x9", "x2", "x1");
          b = b.str("x9", "sp, #24");
        }
      }
      b = b.inst("mov", &format!("x10, #{}", type_tag));
      b = b.str("x10", "sp, #16");
      b.add("sp", "sp", "#16")
    }
    _ => builder,
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  fn emit(inst: Instructions) -> String {
    math_isel(AArch64Builder::new(), &inst).build()
  }
  #[test]
  fn add_lng_emits_proper_instructions() {
    let asm = emit(Instructions::Add(PrimitiveTypes::Lng));
    assert!(asm.contains("Add(Lng)"));
    assert!(asm.contains("add x9, x2, x1"));
  }
}
