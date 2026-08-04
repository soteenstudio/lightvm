/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::primitive_types::PrimitiveTypes;
use std::fs;
use std::io::Result;
pub struct AsmBuilder {
  buffer: String,
}
#[allow(dead_code)]
impl AsmBuilder {
  pub fn new() -> Self {
    Self {
      buffer: String::new(),
    }
  }
  pub fn global(&mut self, name: &str) -> &mut Self {
    self.buffer.push_str(&format!(".global {}\n", name));
    self
  }
  pub fn text(&mut self) -> &mut Self {
    self.buffer.push_str(".text\n");
    self
  }
  pub fn data(&mut self) -> &mut Self {
    self.buffer.push_str(".data\n");
    self
  }
  pub fn rodata(&mut self) -> &mut Self {
    self.buffer.push_str(".section .rodata\n");
    self
  }
  pub fn inject_io_constants(&mut self) -> &mut Self {
    self.buffer.push_str(
      ".section .rodata\n\
       nl_char:    .ascii \"\\n\"\n\
       num_16:     .ascii \"16\"\n\
       obj_str:    .ascii \"[Obj]\"\n\
       arr_str:    .ascii \"[Arr]\"\n\
       cls_str:    .ascii \"\\033[H\\033[J\"\n",
    );
    self
  }
  pub fn label(&mut self, name: &str) -> &mut Self {
    self.buffer.push_str(&format!("{}:\n", name));
    self
  }
  pub fn comment(&mut self, text: &str) -> &mut Self {
    self.buffer.push_str(&format!("    // {}\n", text));
    self
  }
  pub fn inst(&mut self, mnemonic: &str, operands: &str) -> &mut Self {
    if operands.is_empty() {
      self.buffer.push_str(&format!("    {}\n", mnemonic));
    } else {
      self
        .buffer
        .push_str(&format!("    {} {}\n", mnemonic, operands));
    }
    self
  }
  pub fn alloc(&mut self, name: &str, ty: PrimitiveTypes, value: &str) -> &mut Self {
    let sanitized_name = name
      .chars()
      .map(|c| {
        if c.is_alphanumeric() || c == '_' {
          c
        } else {
          '_'
        }
      })
      .collect::<String>();
    match ty {
      PrimitiveTypes::Str => {
        let escaped_value = value
          .replace("\\", "\\\\")
          .replace("\"", "\\\"")
          .replace("\n", "\\n")
          .replace("\r", "\\r")
          .replace("\t", "\\t");
        self.buffer.push_str(&format!(
          "{}:\n    {} \"{}\"\n",
          sanitized_name,
          ty.directive(),
          escaped_value
        ));
      }
      _ => {
        self.buffer.push_str(&format!(
          "{}:\n    {} {}\n",
          sanitized_name,
          ty.directive(),
          value
        ));
      }
    }
    self
  }
  pub fn build(self) -> String {
    self.buffer
  }
  pub fn write_to_file(&self, path: &str) -> Result<()> {
    fs::write(path, &self.buffer)
  }
}
