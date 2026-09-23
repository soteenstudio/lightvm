/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::modules::gazle::utils::{run_pass::run_pass, time_budget::TimeBudget};
use crate::types::{instructions::Instructions, time_budget::TimeBudget as TimeBudgetType};
use crate::utils::get_time_budget::get_time_budget;
pub fn optimize_bytecode(
  mut bytecode: Vec<Instructions>,
  time_budget: TimeBudgetType,
) -> Vec<Instructions> {
  let mut pass_weights: [i32; 9] = [1; 9];
  let budget_ms = get_time_budget(time_budget);
  println!("Budget: {}ms", budget_ms);
  let budget = TimeBudget::new(budget_ms);
  let mut index_mapping = Vec::new();
  let mut scratch = Vec::new();
  loop {
    if budget.is_expired() {
      break;
    }
    let mut changed = false;
    let len_before = bytecode.len();
    let mut order: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    order.sort_by(|a, b| pass_weights[*b].cmp(&pass_weights[*a]));
    for &pass_id in &order {
      if budget.is_expired() {
        break;
      }
      let len_before_pass = bytecode.len();
      let pass_changed = run_pass(pass_id, &mut bytecode);
      let len_after_pass = bytecode.len();
      if pass_changed {
        let reduction = (len_before_pass as i32) - (len_after_pass as i32);
        let reward = if reduction > 0 { reduction * 2 } else { 1 };
        pass_weights[pass_id] += reward;
        changed = true;
      } else {
        pass_weights[pass_id] = pass_weights[pass_id].saturating_sub(1).max(1);
      }
    }
    if budget.is_expired() {
      break;
    }
    let len = bytecode.len();
    index_mapping.clear();
    index_mapping.reserve(len);
    std::mem::swap(&mut bytecode, &mut scratch);
    bytecode.clear();
    bytecode.reserve(len);
    let mut new_idx = 0;
    for (old_idx, instr) in scratch.drain(..).enumerate() {
      let keep = match &instr {
        Instructions::Jump(target) => *target != old_idx + 1,
        Instructions::Nop => false,
        _ => true,
      };
      if keep {
        index_mapping.push(new_idx);
        new_idx += 1;
        bytecode.push(instr);
      } else {
        index_mapping.push(new_idx);
        changed = true;
      }
    }
    for instr in &mut bytecode {
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
  use crate::modules::gazle::utils::run_pass::run_pass;
  use crate::types::{primitive_types::PrimitiveTypes, value::Value};
  use smol_str::SmolStr;
  fn previous_optimizer(
    mut bytecode: Vec<Instructions>,
    time_budget: TimeBudgetType,
  ) -> Vec<Instructions> {
    let mut pass_weights: [i32; 9] = [1; 9];
    let budget_ms = get_time_budget(time_budget);
    println!("Budget: {}ms", budget_ms);
    let budget = TimeBudget::new(budget_ms);
    loop {
      if budget.is_expired() {
        break;
      }
      let mut changed = false;
      let len_before = bytecode.len();
      let mut order: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
      order.sort_by(|a, b| pass_weights[*b].cmp(&pass_weights[*a]));
      for &pass_id in &order {
        if budget.is_expired() {
          break;
        }
        let len_before_pass = bytecode.len();
        let pass_changed = run_pass(pass_id, &mut bytecode);
        let len_after_pass = bytecode.len();
        if pass_changed {
          let reduction = (len_before_pass as i32) - (len_after_pass as i32);
          let reward = if reduction > 0 { reduction * 2 } else { 1 };
          pass_weights[pass_id] += reward;
          changed = true;
        } else {
          pass_weights[pass_id] = pass_weights[pass_id].saturating_sub(1).max(1);
        }
      }
      if budget.is_expired() {
        break;
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
  #[test]
  fn repeated_iterations_preserve_jump_targets_across_removed_instructions() {
    for input in [
      vec![
        Instructions::IfFalse(3),
        Instructions::Nop,
        Instructions::Jump(4),
        Instructions::Nop,
        Instructions::Stop,
      ],
      vec![
        Instructions::Jump(2),
        Instructions::Nop,
        Instructions::IfFalse(4),
        Instructions::Nop,
        Instructions::Stop,
      ],
      vec![
        Instructions::Push(Value::Int16(3)),
        Instructions::IfFalse(4),
        Instructions::Nop,
        Instructions::Jump(5),
        Instructions::Nop,
        Instructions::Stop,
      ],
    ] {
      let expected = previous_optimizer(input.clone(), TimeBudgetType::Cheap);
      assert_eq!(optimize_bytecode(input, TimeBudgetType::Cheap), expected);
    }
  }
  #[test]
  #[ignore]
  fn bench_optimizer_scratch_against_previous_allocation() {
    use std::hint::black_box;
    use std::time::Instant;
    let input = vec![
      Instructions::Push(Value::Int16(3)),
      Instructions::IfFalse(4),
      Instructions::Nop,
      Instructions::Jump(5),
      Instructions::Nop,
      Instructions::Stop,
    ];
    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
      black_box(previous_optimizer(
        black_box(input.clone()),
        TimeBudgetType::Cheap,
      ));
    }
    let before = start.elapsed();
    let start = Instant::now();
    for _ in 0..iterations {
      black_box(optimize_bytecode(
        black_box(input.clone()),
        TimeBudgetType::Cheap,
      ));
    }
    eprintln!(
      "optimizer old: {before:?}, scratch: {:?} ({iterations} runs, debug_assertions={})",
      start.elapsed(),
      cfg!(debug_assertions)
    );
  }
  #[test]
  fn terminates_when_no_pass_reports_a_mutation() {
    let bytecode = vec![Instructions::Stop];
    for pass_id in 0..9 {
      let mut pass_bytecode = bytecode.clone();
      assert!(!run_pass(pass_id, &mut pass_bytecode));
      assert_eq!(pass_bytecode, bytecode);
    }
    assert_eq!(
      optimize_bytecode(bytecode.clone(), TimeBudgetType::Cheap),
      bytecode
    );
  }
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
    let optimized = optimize_bytecode(bytecode, TimeBudgetType::Cheap);
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
