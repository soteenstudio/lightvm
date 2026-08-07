/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::stack::Stack;
use crate::types::value::Value;
use crate::utils::vmerror::VMError;
#[inline(always)]
pub fn access_index_func(stack: &mut Stack, ip: usize) -> Result<(), VMError> {
  let index_val = stack.pop().ok_or(VMError::StackUnderflow {
    ip,
    opcode: "ACCESS_INDEX (index)",
  })?;
  if let Some(top) = stack.last_mut() {
    match &mut *top {
      Value::Array(arr) => {
        // Convert index to usize with proper validation
        let i = match &index_val {
          Value::Int16(v) => {
            if *v < 0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            *v as usize
          }
          Value::Int32(v) => {
            if *v < 0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            *v as usize
          }
          Value::Int64(v) => {
            if *v < 0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            usize::try_from(*v).map_err(|_| VMError::TypeMismatch {
              ip,
              expected: "Number (Index)",
              found: "Invalid Index Type",
            })?
          }
          Value::Int128(v) => {
            if *v < 0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            usize::try_from(*v).map_err(|_| VMError::TypeMismatch {
              ip,
              expected: "Number (Index)",
              found: "Invalid Index Type",
            })?
          }
          Value::Float16(v) => {
            let f = v.to_f32();
            if !f.is_finite() || f.fract() != 0.0 || f < 0.0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            let f_val = f as usize;
            if (f_val as f32) != f {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            f_val
          }
          Value::Float32(v) => {
            if !v.is_finite() || v.fract() != 0.0 || *v < 0.0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            let f_val = *v as usize;
            if (f_val as f32) != *v {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            f_val
          }
          Value::Float64(v) => {
            if !v.is_finite() || v.fract() != 0.0 || *v < 0.0 {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            let f_val = *v as usize;
            if (f_val as f64) != *v {
              return Err(VMError::TypeMismatch {
                ip,
                expected: "Number (Index)",
                found: "Invalid Index Type",
              });
            }
            f_val
          }
          _ => {
            return Err(VMError::TypeMismatch {
              ip,
              expected: "Number (Index)",
              found: "Invalid Index Type",
            });
          }
        };

        if i < arr.len() {
          *top = arr[i].clone();
          Ok(())
        } else {
          Err(VMError::OutOfBounds {
            ip,
            index: i,
            len: arr.len(),
          })
        }
      }
      _ => Err(VMError::TypeMismatch {
        ip,
        expected: "Array",
        found: "Non-Array",
      }),
    }
  } else {
    Err(VMError::StackUnderflow {
      ip,
      opcode: "ACCESS_INDEX (array)",
    })
  }
}
