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
use std::fmt::Write;
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
    write!(self.buffer, ".global {}\n", name).unwrap();
    self
  }
  pub fn symbol_type(&mut self, name: &str, ty: &str) -> &mut Self {
    write!(self.buffer, ".type {}, %{}\n", name, ty).unwrap();
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
      "nl_char:    .ascii \"\\n\"\n\
       num_16:     .ascii \"16\"\n\
       obj_str:    .ascii \"[Obj]\"\n\
       arr_str:    .ascii \"[Arr]\"\n\
       cls_str:    .ascii \"\\033[H\\033[J\"\n",
    );
    self
  }
  pub fn label(&mut self, name: &str) -> &mut Self {
    write!(self.buffer, "{}:\n", name).unwrap();
    self
  }
  pub fn comment(&mut self, text: &str) -> &mut Self {
    write!(self.buffer, "    // {}\n", text).unwrap();
    self
  }
  pub fn inst(&mut self, mnemonic: &str, operands: &str) -> &mut Self {
    if operands.is_empty() {
      write!(self.buffer, "    {}\n", mnemonic).unwrap();
    } else {
      write!(self.buffer, "    {} {}\n", mnemonic, operands).unwrap();
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
        write!(
          self.buffer,
          "{}:\n    {} \"{}\"\n",
          sanitized_name,
          ty.directive(),
          escaped_value
        )
        .unwrap();
      }
      _ => {
        write!(
          self.buffer,
          "{}:\n    {} {}\n",
          sanitized_name,
          ty.directive(),
          value
        )
        .unwrap();
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
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn assembly_preserves_spacing_escapes_and_trailing_newlines() {
    let mut builder = AsmBuilder::new();
    builder
      .global("entry name")
      .symbol_type("entry", "function")
      .label("entry")
      .comment("a\tb")
      .inst("ret", "")
      .inst("mov", "x0, x1")
      .alloc("a-b", PrimitiveTypes::Str, "a\\b\"c\nd\re\tf")
      .alloc("num!", PrimitiveTypes::Int, "-12");
    assert_eq!(
      builder.build().as_bytes(),
      b".global entry name\n.type entry, %function\nentry:\n    // a\tb\n    ret\n    mov x0, x1\na_b:\n    .asciz \"a\\\\b\\\"c\\nd\\re\\tf\"\nnum_:\n    .word -12\n"
    );
  }
  #[test]
  #[ignore]
  fn bench_assembly_direct_buffer_against_formatted_temporary() {
    use std::hint::black_box;
    use std::time::Instant;
    let iterations = 100_000;
    let old_alloc = |buffer: &mut String, name: &str, ty: PrimitiveTypes, value: &str| {
      let sanitized_name = name
        .chars()
        .map(|character| {
          if character.is_alphanumeric() || character == '_' {
            character
          } else {
            '_'
          }
        })
        .collect::<String>();
      if ty == PrimitiveTypes::Str {
        let escaped_value = value
          .replace("\\", "\\\\")
          .replace("\"", "\\\"")
          .replace("\n", "\\n")
          .replace("\r", "\\r")
          .replace("\t", "\\t");
        buffer.push_str(&format!(
          "{}:\n    {} \"{}\"\n",
          sanitized_name,
          ty.directive(),
          escaped_value
        ));
      } else {
        buffer.push_str(&format!(
          "{}:\n    {} {}\n",
          sanitized_name,
          ty.directive(),
          value
        ));
      }
    };
    let start = Instant::now();
    for _ in 0..iterations {
      let mut buffer = String::new();
      buffer.push_str(&format!(".global {}\n", black_box("entry")));
      buffer.push_str(&format!(".type {}, %{}\n", "entry", "function"));
      buffer.push_str(&format!("{}:\n", "entry"));
      buffer.push_str(&format!("    // {}\n", "comment"));
      buffer.push_str(&format!("    {}\n", "ret"));
      buffer.push_str(&format!("    {} {}\n", "mov", "x0, x1"));
      old_alloc(&mut buffer, "name", PrimitiveTypes::Str, "value");
      old_alloc(&mut buffer, "num", PrimitiveTypes::Int, "1");
      black_box(buffer);
    }
    let before = start.elapsed();
    let start = Instant::now();
    for _ in 0..iterations {
      let mut builder = AsmBuilder::new();
      builder
        .global(black_box("entry"))
        .symbol_type("entry", "function")
        .label("entry")
        .comment("comment")
        .inst("ret", "")
        .inst("mov", "x0, x1")
        .alloc("name", PrimitiveTypes::Str, "value")
        .alloc("num", PrimitiveTypes::Int, "1");
      black_box(builder.build());
    }
    eprintln!(
      "assembly format: {before:?}, direct: {:?} ({iterations} runs, debug_assertions={})",
      start.elapsed(),
      cfg!(debug_assertions)
    );
  }
}
