/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

use crate::types::value::Value;
use std::borrow::Cow;
#[inline]
pub fn val_func(vars: &mut Vec<Value>, index: usize) {
  if index >= vars.len() {
    vars.resize(index + 1, Value::Undefined);
  }
  if vars[index] == Value::Undefined {
    vars[index] = Value::Marker(Cow::Borrowed("NoInitExpression"));
  }
}
