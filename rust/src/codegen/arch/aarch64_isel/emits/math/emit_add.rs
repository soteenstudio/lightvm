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
pub fn emit_add(builder: AArch64Builder, num_type: &PrimitiveTypes) -> AArch64Builder {
  let type_tag = match num_type {
    PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct => 0,
    PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl => 2,
    PrimitiveTypes::Str => 3,
  };
  let comment_str = format!("Add({:?})", num_type);
  let mut b = builder.comment(&comment_str);
  match num_type {
    PrimitiveTypes::Hlf => {
      // Convert half to single precision for compatibility
      b = b.ldr("h1", "sp, #8");
      b = b.ldr("h2", "sp, #24");
      b = b.inst("fcvt", "s1, h1");
      b = b.inst("fcvt", "s2, h2");
      b = b.inst("fadd", "s0, s2, s1");
      b = b.inst("fcvt", "h0, s0");
      b = b.inst("str", "h0, [sp, #24]");
    }
    PrimitiveTypes::Flt => {
      b = b.ldr("s1", "sp, #8");
      b = b.ldr("s2", "sp, #24");
      b = b.inst("fadd", "s0, s2, s1");
      b = b.inst("str", "s0, [sp, #24]");
    }
    PrimitiveTypes::Dbl => {
      b = b.ldr("d1", "sp, #8");
      b = b.ldr("d2", "sp, #24");
      b = b.inst("fadd", "d0, d2, d1");
      b = b.inst("str", "d0, [sp, #24]");
    }
    PrimitiveTypes::Sht => {
      b = b.ldr("w1", "sp, #8");
      b = b.ldr("w2", "sp, #24");
      b = b.inst3("add", "w9", "w2, w1");
      b = b.str("w9", "sp, #24");
    }
    PrimitiveTypes::Int => {
      b = b.ldr("w1", "sp, #8");
      b = b.ldr("w2", "sp, #24");
      b = b.inst3("add", "w9", "w2, w1");
      b = b.str("w9", "sp, #24");
    }
    PrimitiveTypes::Oct => {
      b = b.ldr("x1", "sp, #8");
      b = b.ldr("x2", "sp, #24");
      b = b.ldr("x3", "sp, #16");
      b = b.ldr("x4", "sp, #32");
      b = b.inst("adds", "x9, x2, x1");
      b = b.inst3("adcs", "x11", "x4, x3");
      b = b.str("x9", "sp, #24");
      b = b.str("x11", "sp, #32");
    }
    PrimitiveTypes::Lng => {
      b = b.ldr("x1", "sp, #8");
      b = b.ldr("x2", "sp, #24");
      b = b.add("x9", "x2", "x1");
      b = b.str("x9", "sp, #24");
    }
    PrimitiveTypes::Str => {
      // String addition not supported - use Concat instruction instead
      panic!("String addition not supported in emit_add - use Concat instruction");
    }
  }
  b = b.inst("mov", &format!("x10, #{}", type_tag));
  b = b.str("x10", "sp, #16");
  b.add("sp", "sp", "#16")
}
