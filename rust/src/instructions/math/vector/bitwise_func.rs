use crate::instructions::math::bitwise::{
  rotate::{rol_func::rol_values, ror_func::ror_values},
  shift::{shl_func::shl_values, shr_func::shr_values},
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use std::sync::Arc;

type BinaryOp = fn(Value, Value, PrimitiveTypes, usize) -> Result<Value, VMError>;

fn bitwise_vector_values(
  left: Value,
  right: Value,
  num_type: PrimitiveTypes,
  ip: usize,
  op: BinaryOp,
) -> Result<Value, VMError> {
  let left_values = left.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: "Integer",
    found: left.type_of(),
  })?;
  let right_values = right.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: "Integer",
    found: right.type_of(),
  })?;
  if left_values.len() != right_values.len() {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Integer",
      found: "Array",
    });
  }
  if !matches!(
    num_type,
    PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Integer",
      found: num_type.directive(),
    });
  }
  if let Some(value) = left_values
    .iter()
    .chain(right_values.iter())
    .find(|value| !value.is_number())
  {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Integer",
      found: value.type_of(),
    });
  }
  left_values
    .iter()
    .cloned()
    .zip(right_values.iter().cloned())
    .map(|(left, right)| op(left, right, num_type, ip))
    .collect::<Result<Vec<_>, _>>()
    .map(|values| Value::Array(Arc::new(values)))
}

fn bitwise_vector_func(
  stack: &mut Stack,
  num_type: PrimitiveTypes,
  ip: usize,
  opcode: &'static str,
  op: BinaryOp,
) -> Result<(), VMError> {
  if stack.len() < 2 {
    return Err(VMError::StackUnderflow { ip, opcode });
  }
  let result = bitwise_vector_values(
    stack[stack.len() - 2].clone(),
    stack.last().unwrap().clone(),
    num_type,
    ip,
    op,
  )?;
  stack.pop();
  *stack.last_mut().unwrap() = result;
  Ok(())
}

macro_rules! bitwise_vector {
  ($values:ident, $func:ident, $scalar:ident, $opcode:literal) => {
    pub fn $values(
      left: Value,
      right: Value,
      num_type: PrimitiveTypes,
      ip: usize,
    ) -> Result<Value, VMError> {
      bitwise_vector_values(left, right, num_type, ip, $scalar)
    }
    pub fn $func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
      bitwise_vector_func(stack, num_type, ip, $opcode, $scalar)
    }
  };
}

bitwise_vector!(shlv_values, shlv_func, shl_values, "SHLV");
bitwise_vector!(shrv_values, shrv_func, shr_values, "SHRV");
bitwise_vector!(rolv_values, rolv_func, rol_values, "ROLV");
bitwise_vector!(rorv_values, rorv_func, ror_values, "RORV");

#[cfg(test)]
mod tests {
  use super::*;

  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }

  #[test]
  fn supports_integer_directives() {
    for num_type in [
      PrimitiveTypes::Sht,
      PrimitiveTypes::Int,
      PrimitiveTypes::Lng,
      PrimitiveTypes::Oct,
    ] {
      for op in [shlv_values, shrv_values, rolv_values, rorv_values] {
        assert!(
          op(
            array(vec![Value::Int32(1)]),
            array(vec![Value::Int32(1)]),
            num_type,
            0
          )
          .is_ok()
        );
      }
    }
  }

  #[test]
  fn validation_errors_preserve_stack() {
    let cases = [
      (Value::Bool(false), array(vec![]), PrimitiveTypes::Int),
      (
        array(vec![Value::Bool(false)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Int,
      ),
      (
        array(vec![Value::Int32(1)]),
        array(vec![]),
        PrimitiveTypes::Int,
      ),
      (
        array(vec![Value::Int32(1)]),
        array(vec![Value::Int32(1)]),
        PrimitiveTypes::Flt,
      ),
    ];
    for (left, right, num_type) in cases {
      let mut stack = Stack::from_vec(vec![left, right]);
      let original = stack.clone();
      assert!(shlv_func(&mut stack, num_type, 3).is_err());
      assert_eq!(stack, original);
    }
    let mut stack = Stack::from_vec(vec![array(vec![])]);
    let original = stack.clone();
    assert!(matches!(
      shlv_func(&mut stack, PrimitiveTypes::Int, 3),
      Err(VMError::StackUnderflow { .. })
    ));
    assert_eq!(stack, original);
  }
}
