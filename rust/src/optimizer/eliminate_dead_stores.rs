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
use crate::types::usage::Usage;
#[derive(PartialEq, Debug)]
enum Demand {
  Keep,
  Drop,
}
#[inline]
pub fn eliminate_dead_stores(bytecode: &[Instructions], usage: &Usage) -> Vec<Instructions> {
  let mut result = Vec::new();
  let mut stack_demands: Vec<Demand> = Vec::new();
  for inst in bytecode.iter().rev() {
    match inst {
      Instructions::Push(_) | Instructions::Get(_) | Instructions::GetIdx(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            result.push(inst.clone());
          }
        }
      }
      Instructions::Val(_) | Instructions::ValIdx(_) => {
        result.push(inst.clone());
        stack_demands.push(Demand::Keep);
      }
      Instructions::Set(arg) => {
        if !usage.read.contains(arg.as_ref()) {
          stack_demands.push(Demand::Drop);
          continue;
        }
        stack_demands.push(Demand::Keep);
        result.push(inst.clone());
      }
      Instructions::SetIdx(_) => {
        stack_demands.push(Demand::Keep);
        result.push(inst.clone());
      }
      Instructions::Println | Instructions::Print => {
        stack_demands.push(Demand::Keep);
        result.push(inst.clone());
      }
      Instructions::Add(_)
      | Instructions::Sub(_)
      | Instructions::Mul(_)
      | Instructions::Div(_)
      | Instructions::Mod(_)
      | Instructions::Shl(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            stack_demands.push(Demand::Keep);
            result.push(inst.clone());
          } else {
            stack_demands.push(Demand::Drop);
            stack_demands.push(Demand::Drop);
            continue;
          }
        } else {
          continue;
        }
      }
      Instructions::Gt(_)
      | Instructions::Lt(_)
      | Instructions::Ge(_)
      | Instructions::Le(_)
      | Instructions::Eq(_)
      | Instructions::Neq(_)
      | Instructions::And
      | Instructions::Or
      | Instructions::Concat => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            stack_demands.push(Demand::Keep);
            result.push(inst.clone());
          } else {
            stack_demands.push(Demand::Drop);
            stack_demands.push(Demand::Drop);
          }
        }
      }
      Instructions::ToString
      | Instructions::ToInteger
      | Instructions::ToLong
      | Instructions::ToFloat
      | Instructions::ToDouble
      | Instructions::TypeOf
      | Instructions::Length
      | Instructions::InspectObj
      | Instructions::InspectArr => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            result.push(inst.clone());
          } else {
            stack_demands.push(Demand::Drop);
          }
        }
      }
      Instructions::MakeObj(count) | Instructions::MakeArray(count) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            for _ in 0..*count {
              stack_demands.push(Demand::Keep);
            }
            result.push(inst.clone());
          } else {
            for _ in 0..*count {
              stack_demands.push(Demand::Drop);
            }
          }
        }
      }
      Instructions::Access(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            result.push(inst.clone());
          } else {
            stack_demands.push(Demand::Drop);
          }
        }
      }
      Instructions::AccessIndex => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            stack_demands.push(Demand::Keep);
            result.push(inst.clone());
          } else {
            stack_demands.push(Demand::Drop);
            stack_demands.push(Demand::Drop);
          }
        }
      }
      Instructions::SetProp(_) => {
        result.push(inst.clone());
        stack_demands.push(Demand::Keep);
        stack_demands.push(Demand::Keep);
      }
      Instructions::Dup => {
        let d1 = stack_demands.pop().unwrap_or(Demand::Drop);
        let d2 = stack_demands.pop().unwrap_or(Demand::Drop);
        if d1 == Demand::Keep || d2 == Demand::Keep {
          stack_demands.push(Demand::Keep);
          result.push(inst.clone());
        } else {
          stack_demands.push(Demand::Drop);
        }
      }
      Instructions::Inc(arg, _) | Instructions::Dec(arg, _) => {
        if !usage.read.contains(arg.as_ref()) {
          continue;
        }
        result.push(inst.clone());
      }
      Instructions::IncIdx(_, _) | Instructions::DecIdx(_, _) => {
        result.push(inst.clone());
      }
      Instructions::IfFalse(_) => {
        result.push(inst.clone());
        stack_demands.push(Demand::Keep);
      }
      Instructions::Jump(_) | Instructions::Stop | Instructions::Return => {
        result.push(inst.clone());
      }
      _ => {
        result.push(inst.clone());
      }
    }
  }
  result.reverse();
  result
}
