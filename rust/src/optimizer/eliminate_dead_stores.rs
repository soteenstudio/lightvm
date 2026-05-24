/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::instructions::Instructions;
use crate::types::usage::Usage;
#[derive(PartialEq, Debug)]
enum Demand {
  Keep,
  Drop,
}
#[inline]
pub fn eliminate_dead_stores(bytecode: &mut Vec<Instructions>, usage: &Usage) {
  let mut stack_demands: Vec<Demand> = Vec::new();
  for i in (0..bytecode.len()).rev() {
    let inst = &mut bytecode[i];
    match inst {
      Instructions::Push(_) | Instructions::Get(_) | Instructions::GetIdx(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Drop {
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
        }
      }
      Instructions::Val(_) | Instructions::ValIdx(_) => {
        stack_demands.push(Demand::Keep);
      }
      Instructions::Set(arg) => {
        if !usage.read.contains(arg.as_str()) {
          stack_demands.push(Demand::Drop);
          *inst = Instructions::Nop;
          continue;
        }
        stack_demands.push(Demand::Keep);
      }
      Instructions::SetIdx(_) => {
        stack_demands.push(Demand::Keep);
      }
      Instructions::Println | Instructions::Print => {
        stack_demands.push(Demand::Keep);
      }
      Instructions::Add(_)
      | Instructions::Sub(_)
      | Instructions::Mul(_)
      | Instructions::Div(_)
      | Instructions::Mod(_)
      | Instructions::Shl(_)
      | Instructions::Shr(_)
      | Instructions::Ror(_)
      | Instructions::Rol(_)
      | Instructions::Pow(_)
      | Instructions::Powi(_)
      | Instructions::Powf(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            stack_demands.push(Demand::Keep);
          } else {
            stack_demands.push(Demand::Drop);
            stack_demands.push(Demand::Drop);
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
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
      | Instructions::Xor
      | Instructions::Concat => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            stack_demands.push(Demand::Keep);
          } else {
            stack_demands.push(Demand::Drop);
            stack_demands.push(Demand::Drop);
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
        }
      }
      Instructions::Not
      | Instructions::ToString
      | Instructions::ToShort
      | Instructions::ToInteger
      | Instructions::ToLong
      | Instructions::ToOcta
      | Instructions::ToHalf
      | Instructions::ToFloat
      | Instructions::ToDouble
      | Instructions::TypeOf
      | Instructions::Length
      | Instructions::InspectObj
      | Instructions::InspectArr
      | Instructions::Sin(_)
      | Instructions::Cos(_)
      | Instructions::Tan(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
          } else {
            stack_demands.push(Demand::Drop);
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
        }
      }
      Instructions::MakeObj(count) | Instructions::MakeArray(count) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            for _ in 0..*count {
              stack_demands.push(Demand::Keep);
            }
          } else {
            for _ in 0..*count {
              stack_demands.push(Demand::Drop);
            }
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
        }
      }
      Instructions::Access(_) => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
          } else {
            stack_demands.push(Demand::Drop);
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
        }
      }
      Instructions::AccessIndex => {
        if let Some(demand) = stack_demands.pop() {
          if demand == Demand::Keep {
            stack_demands.push(Demand::Keep);
            stack_demands.push(Demand::Keep);
          } else {
            stack_demands.push(Demand::Drop);
            stack_demands.push(Demand::Drop);
            *inst = Instructions::Nop;
          }
        } else {
          *inst = Instructions::Nop;
        }
      }
      Instructions::SetProp(_) => {
        stack_demands.push(Demand::Keep);
        stack_demands.push(Demand::Keep);
      }
      Instructions::Dup => {
        let d1 = stack_demands.pop().unwrap_or(Demand::Drop);
        let d2 = stack_demands.pop().unwrap_or(Demand::Drop);
        if d1 == Demand::Keep || d2 == Demand::Keep {
          stack_demands.push(Demand::Keep);
        } else {
          stack_demands.push(Demand::Drop);
          *inst = Instructions::Nop;
        }
      }
      Instructions::Inc(arg, _) | Instructions::Dec(arg, _) => {
        if !usage.read.contains(arg.as_str()) {
          *inst = Instructions::Nop;
        }
      }
      Instructions::IncIdx(_, _) | Instructions::DecIdx(_, _) => {}
      Instructions::IfFalse(_) => {
        stack_demands.push(Demand::Keep);
      }
      Instructions::Jump(_) | Instructions::Stop | Instructions::Return => {}
      _ => {}
    }
  }
  bytecode.retain(|x| !matches!(x, Instructions::Nop));
}
