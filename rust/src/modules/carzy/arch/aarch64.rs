/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::carzy::asm::AsmBuilder;
use crate::types::primitive_types::PrimitiveTypes;
use std::io::Result;
pub struct AArch64Builder {
  inner: AsmBuilder,
}
#[allow(dead_code)]
impl AArch64Builder {
  pub fn new() -> Self {
    Self {
      inner: AsmBuilder::new(),
    }
  }
  pub fn global(mut self, name: &str) -> Self {
    self.inner.global(name);
    self
  }
  pub fn text(mut self) -> Self {
    self.inner.text();
    self
  }
  pub fn data(mut self) -> Self {
    self.inner.data();
    self
  }
  pub fn rodata(mut self) -> Self {
    self.inner.rodata();
    self
  }
  pub fn inject_io_constants(mut self) -> Self {
    self.inner.inject_io_constants();
    self
  }
  pub fn label(mut self, name: &str) -> Self {
    self.inner.label(name);
    self
  }
  pub fn comment(mut self, text: &str) -> Self {
    self.inner.comment(text);
    self
  }
  pub fn inst(mut self, mnemonic: &str, operands: &str) -> Self {
    self.inner.inst(mnemonic, operands);
    self
  }
  pub fn inst3(mut self, mnemonic: &str, op1: &str, op2: &str) -> Self {
    self.inner.inst(mnemonic, &format!("{}, {}", op1, op2));
    self
  }
  pub fn alloc(mut self, name: &str, ty: PrimitiveTypes, value: &str) -> Self {
    self.inner.alloc(name, ty, value);
    self
  }
  pub fn mov(self, rd: &str, op2: &str) -> Self {
    self.inst("mov", &format!("{}, {}", rd, op2))
  }
  pub fn add(self, rd: &str, rn: &str, op2: &str) -> Self {
    self.inst("add", &format!("{}, {}, {}", rd, rn, op2))
  }
  pub fn sub(self, rd: &str, rn: &str, op2: &str) -> Self {
    self.inst("sub", &format!("{}, {}, {}", rd, rn, op2))
  }
  pub fn ldr(self, rt: &str, address: &str) -> Self {
    self.inst("ldr", &format!("{}, [{}]", rt, address))
  }
  pub fn str(self, rt: &str, address: &str) -> Self {
    self.inst("str", &format!("{}, [{}]", rt, address))
  }
  pub fn ret(self) -> Self {
    self.inst("ret", "")
  }
  pub fn build(self) -> String {
    self.inner.build()
  }
  pub fn write_to_file(self, path: &str) -> Result<()> {
    self.inner.write_to_file(path)
  }
}
