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
use std::sync::atomic::{AtomicUsize, Ordering};
static MOD_LABEL_COUNTER: AtomicUsize = AtomicUsize::new(0);
#[inline]
pub fn emit_mod(builder: AArch64Builder, num_type: &PrimitiveTypes) -> AArch64Builder {
  let label_id = MOD_LABEL_COUNTER.fetch_add(1, Ordering::Relaxed);
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
      b = b.inst("cbz", &format!("w1, .Lmod_by_zero_w_{}", label_id));
      b = b.inst3("sdiv", "w9", "w2, w1");
      b = b.inst3("msub", "w9", "w9, w1, w2");
      b = b.inst("b", &format!(".Lmod_done_w_{}", label_id));
      b = b.label(&format!(".Lmod_by_zero_w_{}", label_id));
      b = b.inst("mov", "w9, #0");
      b = b.label(&format!(".Lmod_done_w_{}", label_id));
      b = b.str("w9", "sp, #24");
    }
    PrimitiveTypes::Lng => {
      b = b.ldr("x1", "sp, #8");
      b = b.ldr("x2", "sp, #24");
      b = b.inst("cbz", &format!("x1, .Lmod_by_zero_x_{}", label_id));
      b = b.inst3("sdiv", "x9", "x2, x1");
      b = b.inst3("msub", "x9", "x9, x1, x2");
      b = b.inst("b", &format!(".Lmod_done_x_{}", label_id));
      b = b.label(&format!(".Lmod_by_zero_x_{}", label_id));
      b = b.inst("mov", "x9, #0");
      b = b.label(&format!(".Lmod_done_x_{}", label_id));
      b = b.str("x9", "sp, #24");
    }
    PrimitiveTypes::Oct => {
      panic!(
        "Mod operation not supported for Oct type - 128-bit modulo requires complex implementation"
      );
    }
    PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl => {
      panic!("Mod operation not supported for floating-point types");
    }
    PrimitiveTypes::Str => {
      panic!("Mod operation not supported for Str type");
    }
  }
  b = b.inst("mov", &format!("x10, #{}", type_tag));
  b = b.str("x10", "sp, #16");
  b.add("sp", "sp", "#16")
}
