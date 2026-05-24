/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::conversion::{
  to_double_func::to_double_func, to_float_func::to_float_func, to_half_func::to_half_func,
  to_integer_func::to_integer_func, to_long_func::to_long_func, to_octa_func::to_octa_func,
  to_short_func::to_short_func, to_string_func::to_string_func,
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
pub fn conversions_dispatch(
  instr: &Instructions,
  stack: &mut [Value],
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::ToString => to_string_func(stack, ip),
    Instructions::ToShort => to_short_func(stack, ip),
    Instructions::ToInteger => to_integer_func(stack, ip),
    Instructions::ToLong => to_long_func(stack, ip),
    Instructions::ToOcta => to_octa_func(stack, ip),
    Instructions::ToHalf => to_half_func(stack, ip),
    Instructions::ToFloat => to_float_func(stack, ip),
    Instructions::ToDouble => to_double_func(stack, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
