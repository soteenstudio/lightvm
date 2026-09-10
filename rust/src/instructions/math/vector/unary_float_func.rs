use crate::instructions::math::{
  exp_func::exp_values,
  logarithm::{ln_func::ln_values, log2_func::log2_values, log10_func::log10_values},
  root::{cbrt_func::cbrt_values, sqrt_func::sqrt_values},
};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use std::sync::Arc;

type UnaryOp = fn(Value, PrimitiveTypes, usize) -> Result<Value, VMError>;

fn unary_vector_values(
  value: Value,
  num_type: PrimitiveTypes,
  ip: usize,
  op: UnaryOp,
) -> Result<Value, VMError> {
  let values = value.as_array().ok_or(VMError::TypeMismatch {
    ip,
    expected: "Float",
    found: value.type_of(),
  })?;
  if !matches!(
    num_type,
    PrimitiveTypes::Hlf | PrimitiveTypes::Flt | PrimitiveTypes::Dbl
  ) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Float",
      found: num_type.directive(),
    });
  }
  if let Some(value) = values.iter().find(|value| !value.is_number()) {
    return Err(VMError::TypeMismatch {
      ip,
      expected: "Float",
      found: value.type_of(),
    });
  }
  values
    .iter()
    .cloned()
    .map(|value| op(value, num_type, ip))
    .collect::<Result<Vec<_>, _>>()
    .map(|values| Value::Array(Arc::new(values)))
}

fn unary_vector_func(
  stack: &mut Stack,
  num_type: PrimitiveTypes,
  ip: usize,
  opcode: &'static str,
  op: UnaryOp,
) -> Result<(), VMError> {
  let value = stack
    .last()
    .cloned()
    .ok_or(VMError::StackUnderflow { ip, opcode })?;
  let result = unary_vector_values(value, num_type, ip, op)?;
  *stack.last_mut().unwrap() = result;
  Ok(())
}

macro_rules! unary_vector {
  ($values:ident, $func:ident, $scalar:ident, $opcode:literal) => {
    pub fn $values(value: Value, num_type: PrimitiveTypes, ip: usize) -> Result<Value, VMError> {
      unary_vector_values(value, num_type, ip, $scalar)
    }

    pub fn $func(stack: &mut Stack, num_type: PrimitiveTypes, ip: usize) -> Result<(), VMError> {
      unary_vector_func(stack, num_type, ip, $opcode, $scalar)
    }
  };
}

unary_vector!(lnv_values, lnv_func, ln_values, "LNV");
unary_vector!(log2v_values, log2v_func, log2_values, "LOG2V");
unary_vector!(log10v_values, log10v_func, log10_values, "LOG10V");
unary_vector!(sqrtv_values, sqrtv_func, sqrt_values, "SQRTV");
unary_vector!(cbrtv_values, cbrtv_func, cbrt_values, "CBRTV");
unary_vector!(expv_values, expv_func, exp_values, "EXPV");

#[cfg(test)]
mod tests {
  use super::*;

  fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(values))
  }

  #[test]
  fn supports_float_directives_and_scalar_domain_behavior() {
    for num_type in [
      PrimitiveTypes::Hlf,
      PrimitiveTypes::Flt,
      PrimitiveTypes::Dbl,
    ] {
      for op in [
        lnv_values,
        log2v_values,
        log10v_values,
        sqrtv_values,
        cbrtv_values,
        expv_values,
      ] {
        assert!(op(array(vec![Value::Int32(1)]), num_type, 0).is_ok());
      }
    }
    let logarithm = lnv_values(array(vec![Value::Float32(-1.0)]), PrimitiveTypes::Flt, 0).unwrap();
    let square_root =
      sqrtv_values(array(vec![Value::Float32(-1.0)]), PrimitiveTypes::Flt, 0).unwrap();
    assert!(matches!(logarithm, Value::Array(values) if values[0].as_f32().is_nan()));
    assert!(matches!(square_root, Value::Array(values) if values[0].as_f32().is_nan()));
  }

  #[test]
  fn validation_errors_preserve_stack() {
    for value in [Value::Bool(false), array(vec![Value::Bool(false)])] {
      let mut stack = Stack::from_vec(vec![value]);
      let original = stack.clone();
      assert!(lnv_func(&mut stack, PrimitiveTypes::Flt, 3).is_err());
      assert_eq!(stack, original);
    }
    let mut stack = Stack::from_vec(vec![array(vec![Value::Float32(1.0)])]);
    let original = stack.clone();
    assert!(lnv_func(&mut stack, PrimitiveTypes::Int, 3).is_err());
    assert_eq!(stack, original);
    let mut stack = Stack::new();
    assert!(matches!(
      lnv_func(&mut stack, PrimitiveTypes::Flt, 3),
      Err(VMError::StackUnderflow { .. })
    ));
  }
}
