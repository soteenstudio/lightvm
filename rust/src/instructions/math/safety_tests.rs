/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use super::{arithmetic, trigonometry, vector};
use crate::modules::vmerror::VMError;
use crate::types::{primitive_types::PrimitiveTypes, stack::Stack, value::Value};
use std::sync::Arc;

type Unary = fn(Value, PrimitiveTypes, usize) -> Result<Value, VMError>;
type Binary = fn(Value, Value, PrimitiveTypes, usize) -> Result<Value, VMError>;
type Instruction = fn(&mut Stack, PrimitiveTypes, usize) -> Result<(), VMError>;
const FLOATS: [PrimitiveTypes; 3] = [
  PrimitiveTypes::Hlf,
  PrimitiveTypes::Flt,
  PrimitiveTypes::Dbl,
];
fn array(values: Vec<Value>) -> Value {
  Value::Array(Arc::new(values))
}
fn domain_error(error: VMError) {
  assert_eq!(error.error_code(), "LVM020");
  assert!(matches!(error, VMError::ValueOutOfRange { ip: 42, .. }));
}

#[test]
fn zero_divisors_after_conversion_preserve_scalar_and_vector_stacks() {
  for (scalar, scalar_func, vector, vector_func, division) in [
    (
      arithmetic::div_func::div_values as Binary,
      arithmetic::div_func::div_func as Instruction,
      vector::arithmetic::divv_func::divv_values as Binary,
      vector::arithmetic::divv_func::divv_func as Instruction,
      true,
    ),
    (
      arithmetic::mod_func::mod_values as Binary,
      arithmetic::mod_func::mod_func as Instruction,
      vector::arithmetic::modv_func::modv_values as Binary,
      vector::arithmetic::modv_func::modv_func as Instruction,
      false,
    ),
  ] {
    let check = |error: VMError| {
      assert_eq!(
        error.error_code(),
        if division { "LVM018" } else { "LVM019" }
      );
      assert!(matches!(
        (&error, division),
        (VMError::DivisionByZero { ip: 42 }, true) | (VMError::ModuloByZero { ip: 42 }, false)
      ));
    };
    for num_type in [
      PrimitiveTypes::Sht,
      PrimitiveTypes::Int,
      PrimitiveTypes::Lng,
      PrimitiveTypes::Oct,
    ]
    .into_iter()
    .chain(FLOATS)
    {
      let integer = matches!(
        num_type,
        PrimitiveTypes::Sht | PrimitiveTypes::Int | PrimitiveTypes::Lng | PrimitiveTypes::Oct
      );
      let one = if integer {
        Value::Int128(1)
      } else {
        Value::Float64(1.0)
      };
      let zeros = if integer {
        vec![
          Value::Int16(0),
          Value::Int32(0),
          Value::Int64(0),
          Value::Int128(0),
        ]
      } else {
        vec![
          Value::Float16(half::f16::ZERO),
          Value::Float32(-0.0),
          Value::Float64(0.0),
          Value::Float64(-0.0),
        ]
      };
      for zero in zeros {
        check(scalar(one.clone(), zero.clone(), num_type, 42).unwrap_err());
        let mut stack = Stack::from_vec(vec![Value::Bool(true), one.clone(), zero.clone()]);
        let original = stack.clone();
        check(scalar_func(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack, original);
        let mut stack = Stack::from_vec(vec![
          Value::Bool(true),
          array(vec![one.clone(), one.clone()]),
          array(vec![one.clone(), zero]),
        ]);
        let original = stack.clone();
        check(vector_func(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack, original);
      }
      assert!(scalar(one.clone(), one.clone(), num_type, 42).is_ok());
      assert!(vector(array(vec![one.clone()]), array(vec![one]), num_type, 42).is_ok());
    }
    for (num_type, input) in [(PrimitiveTypes::Hlf, 1e-12), (PrimitiveTypes::Flt, 1e-50)] {
      check(scalar(Value::Float64(1.0), Value::Float64(input), num_type, 42).unwrap_err());
      check(
        vector(
          array(vec![Value::Float64(1.0)]),
          array(vec![Value::Float64(input)]),
          num_type,
          42,
        )
        .unwrap_err(),
      );
      assert!(
        scalar(
          Value::Float64(1.0),
          Value::Float64(input),
          PrimitiveTypes::Dbl,
          42
        )
        .is_ok()
      );
    }
    for num_type in [
      PrimitiveTypes::Sht,
      PrimitiveTypes::Int,
      PrimitiveTypes::Lng,
    ] {
      check(scalar(Value::Int128(1), Value::Int128(1i128 << 64), num_type, 42).unwrap_err());
    }
  }
}

#[test]
fn bounded_domains_match_and_late_invalid_elements_preserve_stack() {
  for (scalar, vector, instruction, valid, invalid) in [
    (
      trigonometry::inverse::asin_func::asin_values as Unary,
      vector::trigonometry::inverse::asinv_func::asinv_values as Unary,
      vector::trigonometry::inverse::asinv_func::asinv_func as Instruction,
      vec![-1.0, 0.0, 1.0],
      vec![-2.0, 2.0, f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
    ),
    (
      trigonometry::inverse::acos_func::acos_values as Unary,
      vector::trigonometry::inverse::acosv_func::acosv_values as Unary,
      vector::trigonometry::inverse::acosv_func::acosv_func as Instruction,
      vec![-1.0, 0.0, 1.0],
      vec![-2.0, 2.0, f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
    ),
    (
      trigonometry::hyperbolic::inverse::acosh_func::acosh_values as Unary,
      vector::trigonometry::hyperbolic::inverse::acoshv_func::acoshv_values as Unary,
      vector::trigonometry::hyperbolic::inverse::acoshv_func::acoshv_func as Instruction,
      vec![1.0, 2.0, f64::INFINITY],
      vec![0.0, -1.0, f64::NEG_INFINITY, f64::NAN],
    ),
    (
      trigonometry::hyperbolic::inverse::atanh_func::atanh_values as Unary,
      vector::trigonometry::hyperbolic::inverse::atanhv_func::atanhv_values as Unary,
      vector::trigonometry::hyperbolic::inverse::atanhv_func::atanhv_func as Instruction,
      vec![-0.5, 0.0, 0.5],
      vec![
        -1.0,
        1.0,
        -2.0,
        2.0,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NAN,
      ],
    ),
  ] {
    for num_type in FLOATS {
      for &input in &valid {
        let expected = scalar(Value::Float64(input), num_type, 42).unwrap();
        let actual = vector(array(vec![Value::Float64(input)]), num_type, 42).unwrap();
        assert_eq!(actual.as_array().unwrap()[0], expected);
      }
      for &input in &invalid {
        domain_error(scalar(Value::Float64(input), num_type, 42).unwrap_err());
        let operand = array(vec![Value::Float64(valid[0]), Value::Float64(input)]);
        let original = operand.as_array().unwrap();
        let mut stack = Stack::from_vec(vec![Value::Bool(true), operand]);
        domain_error(instruction(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0], Value::Bool(true));
        assert!(Arc::ptr_eq(&stack[1].as_array().unwrap(), &original));
      }
      let input = 1.0 + 1e-8;
      let scalar_result = scalar(Value::Float64(input), num_type, 42);
      let vector_result = vector(array(vec![Value::Float64(input)]), num_type, 42);
      assert_eq!(scalar_result.is_ok(), vector_result.is_ok());
    }
  }
}

#[test]
fn unrestricted_trigonometry_preserves_scalar_vector_behavior() {
  for (scalar, vector) in [
    (
      trigonometry::inverse::atan_func::atan_values as Unary,
      vector::trigonometry::inverse::atanv_func::atanv_values as Unary,
    ),
    (
      trigonometry::hyperbolic::inverse::asinh_func::asinh_values as Unary,
      vector::trigonometry::hyperbolic::inverse::asinhv_func::asinhv_values as Unary,
    ),
    (
      arithmetic::sin_func::sin_values as Unary,
      vector::arithmetic::sinv_func::sinv_values as Unary,
    ),
    (
      arithmetic::cos_func::cos_values as Unary,
      vector::arithmetic::cosv_func::cosv_values as Unary,
    ),
    (
      arithmetic::tan_func::tan_values as Unary,
      vector::arithmetic::tanv_func::tanv_values as Unary,
    ),
    (
      trigonometry::hyperbolic::sinh_func::sinh_values as Unary,
      vector::trigonometry::hyperbolic::sinhv_func::sinhv_values as Unary,
    ),
    (
      trigonometry::hyperbolic::cosh_func::cosh_values as Unary,
      vector::trigonometry::hyperbolic::coshv_func::coshv_values as Unary,
    ),
    (
      trigonometry::hyperbolic::tanh_func::tanh_values as Unary,
      vector::trigonometry::hyperbolic::tanhv_func::tanhv_values as Unary,
    ),
  ] {
    for num_type in FLOATS {
      for input in [
        -10.0,
        -2.0,
        0.0,
        2.0,
        10.0,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NAN,
      ] {
        let expected = scalar(Value::Float64(input), num_type, 42)
          .unwrap()
          .as_f64();
        let actual = vector(array(vec![Value::Float64(input)]), num_type, 42)
          .unwrap()
          .as_array()
          .unwrap()[0]
          .as_f64();
        assert!(actual == expected || (actual.is_nan() && expected.is_nan()));
      }
    }
  }
}

#[test]
fn atan2_zero_pairs_use_domain_errors_after_conversion() {
  use trigonometry::inverse::atan2_func::{atan2_func, atan2_values};
  use vector::trigonometry::inverse::atan2v_func::{atan2v_func, atan2v_values};
  for num_type in FLOATS {
    for y in [0.0, -0.0] {
      for x in [0.0, -0.0] {
        domain_error(atan2_values(Value::Float64(y), Value::Float64(x), num_type, 42).unwrap_err());
        let mut stack = Stack::from_vec(vec![
          Value::Bool(true),
          Value::Float64(x),
          Value::Float64(y),
        ]);
        let original = stack.clone();
        domain_error(atan2_func(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack, original);
        let mut stack = Stack::from_vec(vec![
          Value::Bool(true),
          array(vec![Value::Float64(1.0), Value::Float64(x)]),
          array(vec![Value::Float64(1.0), Value::Float64(y)]),
        ]);
        let original = stack.clone();
        domain_error(atan2v_func(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack, original);
      }
    }
    for (y, x) in [
      (0.0, 1.0),
      (1.0, 0.0),
      (f64::INFINITY, f64::INFINITY),
      (f64::NAN, 1.0),
    ] {
      let scalar = atan2_values(Value::Float64(y), Value::Float64(x), num_type, 42)
        .unwrap()
        .as_f64();
      let vector = atan2v_values(
        array(vec![Value::Float64(y)]),
        array(vec![Value::Float64(x)]),
        num_type,
        42,
      )
      .unwrap()
      .as_array()
      .unwrap()[0]
        .as_f64();
      assert!(scalar == vector || (scalar.is_nan() && vector.is_nan()));
    }
  }
  for num_type in [PrimitiveTypes::Hlf, PrimitiveTypes::Flt] {
    domain_error(
      atan2_values(Value::Float64(1e-50), Value::Float64(-1e-50), num_type, 42).unwrap_err(),
    );
    domain_error(
      atan2v_values(
        array(vec![Value::Float64(1e-50)]),
        array(vec![Value::Float64(-1e-50)]),
        num_type,
        42,
      )
      .unwrap_err(),
    );
  }
}

#[test]
fn domain_boundaries_are_checked_at_requested_precision() {
  for num_type in FLOATS {
    for (scalar, vector, input, valid) in [
      (
        trigonometry::inverse::asin_func::asin_values as Unary,
        vector::trigonometry::inverse::asinv_func::asinv_values as Unary,
        1.0 + 1e-8,
        !matches!(num_type, PrimitiveTypes::Dbl),
      ),
      (
        trigonometry::inverse::acos_func::acos_values as Unary,
        vector::trigonometry::inverse::acosv_func::acosv_values as Unary,
        -1.0 - 1e-8,
        !matches!(num_type, PrimitiveTypes::Dbl),
      ),
      (
        trigonometry::hyperbolic::inverse::acosh_func::acosh_values as Unary,
        vector::trigonometry::hyperbolic::inverse::acoshv_func::acoshv_values as Unary,
        1.0 - 1e-8,
        !matches!(num_type, PrimitiveTypes::Dbl),
      ),
      (
        trigonometry::hyperbolic::inverse::atanh_func::atanh_values as Unary,
        vector::trigonometry::hyperbolic::inverse::atanhv_func::atanhv_values as Unary,
        1.0 - 1e-8,
        matches!(num_type, PrimitiveTypes::Dbl),
      ),
    ] {
      for result in [
        scalar(Value::Float64(input), num_type, 42),
        vector(array(vec![Value::Float64(input)]), num_type, 42),
      ] {
        if valid {
          assert!(result.is_ok());
        } else {
          domain_error(result.unwrap_err());
        }
      }
    }
  }
}

#[test]
fn diagnostics_describe_exclusive_and_pair_domains() {
  let error = trigonometry::hyperbolic::inverse::atanh_func::atanh_values(
    Value::Float64(1.0),
    PrimitiveTypes::Dbl,
    42,
  )
  .unwrap_err();
  assert!(error.to_string().contains("exclusive"));
  assert!(error.diagnostic_link().ends_with("lvm020-code"));
  let error = trigonometry::inverse::atan2_func::atan2_values(
    Value::Float64(-0.0),
    Value::Float64(0.0),
    PrimitiveTypes::Dbl,
    42,
  )
  .unwrap_err();
  assert!(
    error
      .to_string()
      .contains("Operands must not both be zero, including signed zero")
  );
  domain_error(error);
}

#[test]
fn scalar_domain_failure_preserves_stack_in_every_precision() {
  for instruction in [
    trigonometry::inverse::asin_func::asin_func as Instruction,
    trigonometry::inverse::acos_func::acos_func as Instruction,
    trigonometry::hyperbolic::inverse::acosh_func::acosh_func as Instruction,
    trigonometry::hyperbolic::inverse::atanh_func::atanh_func as Instruction,
  ] {
    for num_type in FLOATS {
      for input in [f64::NAN, f64::NEG_INFINITY] {
        let mut stack = Stack::from_vec(vec![Value::Bool(true), Value::Float64(input)]);
        domain_error(instruction(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0], Value::Bool(true));
        assert_eq!(stack[1].as_f64().to_bits(), input.to_bits());
      }
    }
  }
}
