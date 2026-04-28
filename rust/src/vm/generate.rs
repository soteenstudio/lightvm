/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use crate::Value;
pub fn generate(bytecode: Vec<Instructions>) -> String {
  let mut ir = String::new();
  let mut stack: Vec<String> = Vec::new();
  let mut v_counter = 0;
  ir.push_str("function u0:0() -> i64 {\nblock0:\n");
  for instr in bytecode {
    match instr {
      Instructions::Push(val) => {
        let v = format!("v{}", v_counter);
        match val {
          Value::Int64(n) => ir.push_str(&format!("    {} = iconst.i64 {}\n", v, n)),
          Value::Float64(f) => ir.push_str(&format!("    {} = f64const {}\n", v, f)),
          Value::Bool(b) => ir.push_str(&format!(
            "    {} = iconst.i8 {}\n",
            v,
            if b { 1 } else { 0 }
          )),
          _ => continue,
        }
        stack.push(v);
        v_counter += 1;
      }
      Instructions::Set(name) => {
        if let Some(v_src) = stack.pop() {
          let slot = vars.entry(name.clone()).or_insert_with(|| {
            let s = format!("ss{}", slot_counter);
            ir.insert_str(0, &format!("    ss{} = stack_slot 8\n", slot_counter));
            slot_counter += 1;
            s
          });
          ir.push_str(&format!("    stack_store {}, {}\n", v_src, slot));
        }
      }
      Instructions::Get(name) => {
        if let Some(slot) = vars.get(&name) {
          let v = format!("v{}", v_counter);
          ir.push_str(&format!("    {} = stack_load.i64 {}\n", v, slot));
          stack.push(v);
          v_counter += 1;
        }
      }
      Instructions::Add(_)
      | Instructions::Sub(_)
      | Instructions::Mul(_)
      | Instructions::Div(_)
      | Instructions::Mod(_) => {
        let v_b = stack.pop().expect("Stack underflow b");
        let v_a = stack.pop().expect("Stack underflow a");
        let v_res = format!("v{}", v_counter);
        let op = match instr {
          Instructions::Add(_) => "iadd",
          Instructions::Sub(_) => "isub",
          Instructions::Mul(_) => "imul",
          Instructions::Div(_) => "sdiv",
          Instructions::Mod(_) => "srem",
          _ => unreachable!(),
        };
        ir.push_str(&format!("    {} = {} {}, {}\n", v_res, op, v_a, v_b));
        stack.push(v_res);
        v_counter += 1;
      }
      Instructions::Eq(_) | Instructions::Gt(_) | Instructions::Lt(_) => {
        let v_b = stack.pop().expect("Stack underflow b");
        let v_a = stack.pop().expect("Stack underflow a");
        let v_res = format!("v{}", v_counter);
        let cond = match instr {
          Instructions::Eq(_) => "icmp eq",
          Instructions::Gt(_) => "icmp sgt",
          Instructions::Lt(_) => "icmp slt",
          _ => "icmp eq",
        };
        ir.push_str(&format!("    {} = {} {}, {}\n", v_res, cond, v_a, v_b));
        stack.push(v_res);
        v_counter += 1;
      }
      Instructions::Println | Instructions::Print => {
        if let Some(v_val) = stack.pop() {
          ir.push_str("    fn1 = u0:1\n");
          ir.push_str(&format!("    call fn1({})\n", v_val));
        }
      }
      Instructions::Return => {
        if let Some(v_last) = stack.pop() {
          ir.push_str(&format!("    return {}\n", v_last));
        } else {
          ir.push_str("    return\n");
        }
        break;
      }
      _ => {}
    }
  }
  ir.push_str("}\n");
  ir
}
