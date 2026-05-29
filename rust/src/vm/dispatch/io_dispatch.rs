/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::instructions::io::{
  clear_screen_func::clear_screen_func,
  inspect_arr_func::inspect_arr_func,
  inspect_obj_func::inspect_obj_func,
  print_func::print_func,
  println_func::println_func,
  stdin_func::stdin_func,
  stdout_func::{stdout_func, stdoutln_func},
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
use smallvec::SmallVec;
#[inline(always)]
pub fn io_dispatch(
  instr: &Instructions,
  stack: &mut SmallVec<[Value; 16]>,
  ip: usize,
) -> Result<(), VMError> {
  match instr {
    Instructions::Print => print_func(stack, ip),
    Instructions::Println => println_func(stack, ip),
    Instructions::Stdout => stdout_func(stack, ip),
    Instructions::Stdoutln => stdoutln_func(stack, ip),
    Instructions::Stdin => stdin_func(stack),
    Instructions::InspectObj => inspect_obj_func(stack, ip),
    Instructions::InspectArr => inspect_arr_func(stack, ip),
    Instructions::ClearScreen => clear_screen_func(),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
