/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

use crate::instructions::io::{
  inspect_arr_func::inspect_arr_func, inspect_obj_func::inspect_obj_func, print_func::print_func,
  println_func::println_func,
};
use crate::types::{instructions::Instructions, value::Value};
use crate::utils::vmerror::VMError;
pub fn io_dispatch(instr: &Instructions, stack: &mut Vec<Value>, ip: usize) -> Result<(), VMError> {
  match instr {
    Instructions::Print => print_func(stack, ip),
    Instructions::Println => println_func(stack, ip),
    Instructions::InspectObj => inspect_obj_func(stack, ip),
    Instructions::InspectArr => inspect_arr_func(stack, ip),
    _ => unsafe { std::hint::unreachable_unchecked() },
  }
}
