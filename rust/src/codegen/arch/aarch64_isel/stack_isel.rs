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
pub fn stack_isel(mut builder: AArch64Builder, inst: &Instructions) -> AArch64Builder {
  match inst {
    Instructions::InitStack(size) => {
      // InitStack is now handled in the prologue; skip it here
      builder.comment(&format!(
        "InitStack({}) handled in function prologue",
        size
      ))
    }
    Instructions::PushInt16(val) => {
      let v = *val as i64 as u64; // Sign-extend then convert to u64
      let low = v & 0xFFFF;
      let mid1 = (v >> 16) & 0xFFFF;
      let mid2 = (v >> 32) & 0xFFFF;
      let high = (v >> 48) & 0xFFFF;
      builder = builder
        .comment(&format!("PushInt16({})", val))
        .inst("movz", &format!("x9, #{:#x}", low));
      // Always emit movk for non-zero chunks (includes 0xFFFF for negative numbers)
      if mid1 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #16", mid1));
      }
      if mid2 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #32", mid2));
      }
      if high != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #48", high));
      }
      builder.sub("sp", "sp", "#16").str("x9", "sp")
    }
    Instructions::PushInt32(val) => {
      let v = *val as i64 as u64; // Sign-extend then convert to u64
      let low = v & 0xFFFF;
      let mid1 = (v >> 16) & 0xFFFF;
      let mid2 = (v >> 32) & 0xFFFF;
      let high = (v >> 48) & 0xFFFF;
      builder = builder
        .comment(&format!("PushInt32({})", val))
        .inst("movz", &format!("x9, #{:#x}", low));
      // Always emit movk for non-zero chunks (includes 0xFFFF for negative numbers)
      if mid1 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #16", mid1));
      }
      if mid2 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #32", mid2));
      }
      if high != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #48", high));
      }
      builder.sub("sp", "sp", "#16").str("x9", "sp")
    }
    Instructions::PushInt64(val) => {
      let v = *val as u64;
      let low = v & 0xFFFF;
      let mid1 = (v >> 16) & 0xFFFF;
      let mid2 = (v >> 32) & 0xFFFF;
      let high = (v >> 48) & 0xFFFF;
      builder = builder
        .comment(&format!("PushInt64({})", val))
        .inst("movz", &format!("x9, #{:#x}", low));
      if mid1 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #16", mid1));
      }
      if mid2 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #32", mid2));
      }
      if high != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #48", high));
      }
      builder.sub("sp", "sp", "#16").str("x9", "sp")
    }
    Instructions::PushInt128(val) => {
      let v = *val as u128;
      // Build low 64 bits in x9
      let low_64 = (v & 0xFFFFFFFFFFFFFFFF) as u64;
      let low = low_64 & 0xFFFF;
      let mid1 = (low_64 >> 16) & 0xFFFF;
      let mid2 = (low_64 >> 32) & 0xFFFF;
      let high = (low_64 >> 48) & 0xFFFF;
      builder = builder
        .comment(&format!("PushInt128({})", val))
        .inst("movz", &format!("x9, #{:#x}", low));
      if mid1 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #16", mid1));
      }
      if mid2 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #32", mid2));
      }
      if high != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #48", high));
      }
      // Build high 64 bits in x10
      let high_64 = (v >> 64) as u64;
      let low_h = high_64 & 0xFFFF;
      let mid1_h = (high_64 >> 16) & 0xFFFF;
      let mid2_h = (high_64 >> 32) & 0xFFFF;
      let high_h = (high_64 >> 48) & 0xFFFF;
      builder = builder.inst("movz", &format!("x10, #{:#x}", low_h));
      if mid1_h != 0 {
        builder = builder.inst("movk", &format!("x10, #{:#x}, lsl #16", mid1_h));
      }
      if mid2_h != 0 {
        builder = builder.inst("movk", &format!("x10, #{:#x}, lsl #32", mid2_h));
      }
      if high_h != 0 {
        builder = builder.inst("movk", &format!("x10, #{:#x}, lsl #48", high_h));
      }
      // Store both registers into 16-byte stack slot
      builder
        .sub("sp", "sp", "#16")
        .str("x9", "sp")
        .inst("str", "x10, [sp, #8]")
    }
    Instructions::PushFloat16(val) => {
      let bits = val.to_bits() as u64;
      builder
        .comment(&format!("PushFloat16({})", val))
        .inst("movz", &format!("x9, #{:#x}", bits))
        .inst("fmov", "d0, x9")
        .sub("sp", "sp", "#16")
        .inst("str", "d0, [sp]")
    }
    Instructions::PushFloat32(val) => {
      let bits = val.to_bits() as u64;
      builder
        .comment(&format!("PushFloat32({})", val))
        .inst("movz", &format!("x9, #{:#x}", bits & 0xFFFF))
        .inst(
          "movk",
          &format!("x9, #{:#x}, lsl #16", (bits >> 16) & 0xFFFF),
        )
        .inst3("fmov", "s0", "w9")
        .sub("sp", "sp", "#16")
        .inst("str", "s0, [sp]")
    }
    Instructions::PushFloat64(val) => {
      let bits = val.to_bits();
      let low = bits & 0xFFFF;
      let mid1 = (bits >> 16) & 0xFFFF;
      let mid2 = (bits >> 32) & 0xFFFF;
      let high = (bits >> 48) & 0xFFFF;
      builder = builder
        .comment(&format!("PushFloat64({})", val))
        .inst("movz", &format!("x9, #{:#x}", low));
      if mid1 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #16", mid1));
      }
      if mid2 != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #32", mid2));
      }
      if high != 0 {
        builder = builder.inst("movk", &format!("x9, #{:#x}, lsl #48", high));
      }
      builder
        .inst("fmov", "d0, x9")
        .sub("sp", "sp", "#16")
        .inst("str", "d0, [sp]")
    }
    Instructions::PushString(s) => builder
      .comment(&format!(
        "PushString(\"{}\") - TODO: implement runtime string allocation",
        s
      ))
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::PushArray(arr) => builder
      .comment(&format!(
        "PushArray(len={}) - TODO: implement runtime array allocation",
        arr.len()
      ))
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::PushObject(obj) => builder
      .comment(&format!(
        "PushObject(len={}) - TODO: implement runtime object allocation",
        obj.len()
      ))
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::PushBool(b) => {
      let val = if *b { 1 } else { 0 };
      builder
        .comment(&format!("PushBool({})", b))
        .inst("movz", &format!("x9, #{}", val))
        .sub("sp", "sp", "#16")
        .str("x9", "sp")
    }
    Instructions::PushNull => builder
      .comment("PushNull")
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::PushUndefined => builder
      .comment("PushUndefined")
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::PushNaN => builder
      .comment("PushNaN - TODO: implement proper NaN representation")
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::Push(_) => builder
      .comment("Push (Generic) - TODO: implement proper value representation")
      .inst("mov", "x9, #0")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::ValIdx(idx) => {
      let offset = idx * 16;
      builder
        .comment(&format!("ValIdx({})", idx))
        .ldr("x9", "sp")
        .inst("str", &format!("x9, [x19, #-{}]", offset + 16))
    }
    Instructions::SetIdx(idx) => {
      let offset = idx * 16;
      builder
        .comment(&format!("SetIdx({})", idx))
        .ldr("x9", "sp")
        .inst("str", &format!("x9, [x19, #-{}]", offset + 16))
    }
    Instructions::GetIdx(idx) => {
      let offset = idx * 16;
      builder
        .comment(&format!("GetIdx({})", idx))
        .inst("ldr", &format!("x9, [x19, #-{}]", offset + 16))
        .sub("sp", "sp", "#16")
        .str("x9", "sp")
    }
    Instructions::Concat => builder
      .comment("Concat - TODO: implement runtime concat operation")
      .comment("Pop two values, concatenate, push result")
      .ldr("x9", "sp")
      .inst("ldr", "x10, [sp, #16]")
      .inst("add", "sp, sp, #16")
      .comment("Placeholder: just push first value for now")
      .str("x9", "sp"),
    Instructions::Dup => builder
      .comment("Dup")
      .ldr("x9", "sp")
      .sub("sp", "sp", "#16")
      .str("x9", "sp"),
    Instructions::Swap => builder
      .comment("Swap")
      .inst("ldr", "x9, [sp]")
      .inst("ldr", "x10, [sp, #16]")
      .inst("str", "x9, [sp, #16]")
      .inst("str", "x10, [sp]"),
    Instructions::Truncate => builder
      .comment("Truncate - TODO: implement runtime truncate operation")
      .comment("Pop value, truncate, push result")
      .ldr("x9", "sp")
      .comment("Placeholder: just keep value as-is for now")
      .str("x9", "sp"),
    _ => builder,
  }
}
