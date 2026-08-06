/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::codegen::arch::aarch64_isel::emits::math::{
  emit_add::emit_add, emit_div::emit_div, emit_mod::emit_mod, emit_mul::emit_mul,
  emit_sub::emit_sub,
};
use crate::modules::carzy::arch::aarch64::AArch64Builder;
use crate::types::instructions::Instructions;
pub fn math_isel(builder: AArch64Builder, inst: &Instructions) -> AArch64Builder {
  match inst {
    Instructions::Add(num_type) => emit_add(builder, num_type),
    Instructions::Sub(num_type) => emit_sub(builder, num_type),
    Instructions::Mul(num_type) => emit_mul(builder, num_type),
    Instructions::Div(num_type) => emit_div(builder, num_type),
    Instructions::Mod(num_type) => emit_mod(builder, num_type),
    _ => builder,
  }
}
