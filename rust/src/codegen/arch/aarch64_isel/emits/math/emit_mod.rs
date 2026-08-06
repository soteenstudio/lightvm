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
use crate::types::primitive_types::PrimitiveTypes;
#[inline]
pub fn emit_mod(builder: AArch64Builder, num_type: &PrimitiveTypes) -> AArch64Builder {
  let type_tag = match num_type {
    PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct => 0,
    PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl => 2,
    PrimitiveTypes::Str => 3,
  };
  let comment_str = format!("Mod({:?})", num_type);
  let mut b = builder.comment(&comment_str);
  match num_type {
    PrimitiveTypes::Sht | PrimitiveTypes::Int => {
      b = b.ldr("w1", "sp, #8");
      b = b.ldr("w2", "sp, #24");
      // Check for division by zero
      b = b.inst("cbz", "w1, 1f");
      b = b.inst3("sdiv", "w9", "w2, w1");
      b = b.inst3("msub", "w9", "w9, w1, w2");
      b = b.str("w9", "sp, #24");
      b = b.inst("b", "2f");
      b = b.label("1");
      // Trigger division by zero trap
      b = b.inst("udiv", "w9, w9, wzr");
      b = b.label("2");
    }
    PrimitiveTypes::Lng => {
      b = b.ldr("x1", "sp, #8");
      b = b.ldr("x2", "sp, #24");
      // Check for division by zero
      b = b.inst("cbz", "x1, 1f");
      b = b.inst3("sdiv", "x9", "x2, x1");
      b = b.inst3("msub", "x9", "x9, x1, x2");
      b = b.str("x9", "sp, #24");
      b = b.inst("b", "2f");
      b = b.label("1");
      // Trigger division by zero trap
      b = b.inst("udiv", "x9, x9, xzr");
      b = b.label("2");
    }
    PrimitiveTypes::Oct => {
      // 128-bit modulo not supported - would require software division
      panic!("128-bit integer modulo not supported");
    }
    PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl => {
      panic!("Floating-point modulo not supported");
    }
    PrimitiveTypes::Str => {
      panic!("String modulo not supported");
    }
  }
  b = b.inst("mov", &format!("x10, #{}", type_tag));
  b = b.str("x10", "sp, #16");
  b.add("sp", "sp", "#16")
}
