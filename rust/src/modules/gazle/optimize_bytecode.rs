/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::gazle::utils::run_pass::run_pass;
use crate::types::instructions::Instructions;
pub fn optimize_bytecode(mut bytecode: Vec<Instructions>) -> Vec<Instructions> {
  let mut pass_weights: [i32; 9] = [1; 9];
  loop {
    let mut changed = false;
    let len_before = bytecode.len();
    let mut order: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    order.sort_by(|a, b| pass_weights[*b].cmp(&pass_weights[*a]));
    for &pass_id in &order {
      let len_before_pass = bytecode.len();
      let prev_bytes_debug = bytecode.clone();
      run_pass(pass_id, &mut bytecode);
      let len_after_pass = bytecode.len();
      if bytecode != prev_bytes_debug {
        let reduction = (len_before_pass as i32) - (len_after_pass as i32);
        let reward = if reduction > 0 { reduction * 2 } else { 1 };
        pass_weights[pass_id] += reward;
        changed = true;
      } else {
        pass_weights[pass_id] = pass_weights[pass_id].saturating_sub(1).max(1);
      }
    }
    let len = bytecode.len();
    let mut index_mapping = Vec::with_capacity(len);
    let mut new_bytecode = Vec::with_capacity(len);
    let mut new_idx = 0;
    for (old_idx, instr) in bytecode.into_iter().enumerate() {
      let keep = match &instr {
        Instructions::Jump(target) => *target != old_idx + 1,
        Instructions::Nop => false,
        _ => true,
      };
      if keep {
        index_mapping.push(new_idx);
        new_idx += 1;
        new_bytecode.push(instr);
      } else {
        index_mapping.push(new_idx);
        changed = true;
      }
    }
    for instr in &mut new_bytecode {
      match instr {
        Instructions::Jump(target) | Instructions::IfFalse(target)
          if *target < index_mapping.len() =>
        {
          let mapped = index_mapping[*target];
          if *target != mapped {
            *target = mapped;
            changed = true;
          }
        }
        _ => {}
      }
    }
    bytecode = new_bytecode;
    if len_before != bytecode.len() {
      changed = true;
    }
    if !changed {
      break;
    }
  }
  bytecode
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::{primitive_types::PrimitiveTypes, value::Value};
  use smol_str::SmolStr;
  #[test]
  fn folds_constant_add_and_concat_to_correct_result() {
    let bytecode = vec![
      Instructions::Val(SmolStr::new("x")),
      Instructions::Push(Value::Int16(10)),
      Instructions::Set(SmolStr::new("x")),
      Instructions::Val(SmolStr::new("y")),
      Instructions::Push(Value::Int16(5)),
      Instructions::Set(SmolStr::new("y")),
      Instructions::PushString(SmolStr::new("Result is: ")),
      Instructions::Get(SmolStr::new("x")),
      Instructions::Get(SmolStr::new("y")),
      Instructions::Add(PrimitiveTypes::Sht),
      Instructions::Concat,
      Instructions::Println,
    ];
    let optimized = optimize_bytecode(bytecode);
    assert_eq!(
      optimized,
      vec![
        Instructions::PushString(SmolStr::new("Result is: 15")),
        Instructions::Println,
      ]
    );
    assert!(!optimized.contains(&Instructions::Get(SmolStr::new("x"))));
    assert!(!optimized.contains(&Instructions::Get(SmolStr::new("y"))));
    assert!(!optimized.contains(&Instructions::Add(PrimitiveTypes::Sht)));
  }
}
