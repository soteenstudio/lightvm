/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use super::{arithmetic, logarithm, root, trigonometry, vector};
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

fn restricted_unary_operations() -> [(Unary, Instruction, Unary, Instruction, bool); 4] {
  [
    (
      logarithm::ln_func::ln_values,
      logarithm::ln_func::ln_func,
      vector::logarithm::lnv_func::lnv_values,
      vector::logarithm::lnv_func::lnv_func,
      false,
    ),
    (
      logarithm::log2_func::log2_values,
      logarithm::log2_func::log2_func,
      vector::logarithm::log2v_func::log2v_values,
      vector::logarithm::log2v_func::log2v_func,
      false,
    ),
    (
      logarithm::log10_func::log10_values,
      logarithm::log10_func::log10_func,
      vector::logarithm::log10v_func::log10v_values,
      vector::logarithm::log10v_func::log10v_func,
      false,
    ),
    (
      root::sqrt_func::sqrt_values,
      root::sqrt_func::sqrt_func,
      vector::root::sqrtv_func::sqrtv_values,
      vector::root::sqrtv_func::sqrtv_func,
      true,
    ),
  ]
}

#[test]
fn logarithm_and_sqrt_domains_preserve_operands() {
  for (scalar, instruction, vector, vector_instruction, sqrt) in restricted_unary_operations() {
    for num_type in FLOATS {
      let mut invalid = vec![-1.0, f64::NEG_INFINITY, f64::NAN];
      if !sqrt {
        invalid.extend([0.0, -0.0]);
      }
      for input in invalid {
        domain_error(scalar(Value::Float64(input), num_type, 42).unwrap_err());
        let mut stack = Stack::from_vec(vec![Value::Bool(true), Value::Float64(input)]);
        domain_error(instruction(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0], Value::Bool(true));
        assert_eq!(stack[1].as_f64().to_bits(), input.to_bits());
        let operand = array(vec![Value::Float64(1.0), Value::Float64(input)]);
        let original = operand.as_array().unwrap();
        let mut stack = Stack::from_vec(vec![Value::Bool(true), operand]);
        domain_error(vector_instruction(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0], Value::Bool(true));
        assert!(Arc::ptr_eq(&stack[1].as_array().unwrap(), &original));
      }
      for input in [1.0, 4.0, f64::INFINITY] {
        let expected = scalar(Value::Float64(input), num_type, 42).unwrap();
        let actual = vector(array(vec![Value::Float64(input)]), num_type, 42).unwrap();
        assert_eq!(actual.as_array().unwrap()[0], expected);
      }
      for input in [0.0, -0.0] {
        if sqrt {
          for result in [
            scalar(Value::Float64(input), num_type, 42).unwrap(),
            vector(array(vec![Value::Float64(input)]), num_type, 42)
              .unwrap()
              .as_array()
              .unwrap()[0]
              .clone(),
          ] {
            assert_eq!(result.as_f64().to_bits(), input.to_bits());
          }
        }
      }
      for input in [1e-50, -1e-50] {
        let valid = if sqrt {
          input > 0.0 || !matches!(num_type, PrimitiveTypes::Dbl)
        } else {
          input > 0.0 && matches!(num_type, PrimitiveTypes::Dbl)
        };
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
      assert!(matches!(
        vector(
          array(vec![Value::Float64(-1.0), Value::Bool(false)]),
          num_type,
          42
        ),
        Err(VMError::TypeMismatch { ip: 42, .. })
      ));
      let mut stack = Stack::new();
      assert!(matches!(
        instruction(&mut stack, num_type, 42),
        Err(VMError::StackUnderflow { ip: 42, .. })
      ));
      assert!(stack.is_empty());
      assert!(matches!(
        vector_instruction(&mut stack, num_type, 42),
        Err(VMError::StackUnderflow { ip: 42, .. })
      ));
      assert!(stack.is_empty());
    }
    assert!(matches!(
      scalar(Value::Float64(-1.0), PrimitiveTypes::Int, 42),
      Err(VMError::TypeMismatch { ip: 42, .. })
    ));
    assert!(matches!(
      vector(array(vec![Value::Float64(-1.0)]), PrimitiveTypes::Int, 42),
      Err(VMError::TypeMismatch { ip: 42, .. })
    ));
  }
}

#[test]
fn power_domains_validate_converted_pairs_and_preserve_stacks() {
  for (scalar, instruction, vector, vector_instruction, integer_exponent, precisions) in [
    (
      arithmetic::powf_func::powf_values as Binary,
      arithmetic::powf_func::powf_func as Instruction,
      vector::arithmetic::powfv_func::powfv_values as Binary,
      vector::arithmetic::powfv_func::powfv_func as Instruction,
      false,
      FLOATS.to_vec(),
    ),
    (
      arithmetic::powi_func::powi_values as Binary,
      arithmetic::powi_func::powi_func as Instruction,
      vector::arithmetic::powiv_func::powiv_values as Binary,
      vector::arithmetic::powiv_func::powiv_func as Instruction,
      true,
      FLOATS.to_vec(),
    ),
    (
      arithmetic::pow_func::pow_values as Binary,
      arithmetic::pow_func::pow_func as Instruction,
      vector::arithmetic::powv_func::powv_values as Binary,
      vector::arithmetic::powv_func::powv_func as Instruction,
      true,
      vec![
        PrimitiveTypes::Sht,
        PrimitiveTypes::Int,
        PrimitiveTypes::Lng,
        PrimitiveTypes::Oct,
      ],
    ),
  ] {
    for num_type in precisions {
      let floating_base = FLOATS.contains(&num_type);
      let base_value = |base| {
        if floating_base {
          Value::Float64(base)
        } else {
          Value::Int128(base as i128)
        }
      };
      let exponent_value = |exponent| {
        if integer_exponent {
          Value::Int128(exponent as i128)
        } else {
          Value::Float64(exponent)
        }
      };
      let mut invalid = vec![(0.0, -1.0), (-0.0, -2.0)];
      if !integer_exponent {
        invalid.extend([(-2.0, 0.5), (-2.0, -0.5)]);
      }
      for (base, exponent) in invalid {
        let a = base_value(base);
        let b = exponent_value(exponent);
        domain_error(scalar(a.clone(), b.clone(), num_type, 42).unwrap_err());
        let mut stack = Stack::from_vec(vec![Value::Bool(true), a.clone(), b.clone()]);
        let original = stack.clone();
        domain_error(instruction(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack, original);
        let mut stack = Stack::from_vec(vec![
          Value::Bool(true),
          array(vec![base_value(2.0), a]),
          array(vec![exponent_value(2.0), b]),
        ]);
        let original = stack.clone();
        domain_error(vector_instruction(&mut stack, num_type, 42).unwrap_err());
        assert_eq!(stack, original);
      }
      for (base, exponent) in [(-2.0, 3.0), (-2.0, -2.0), (0.0, 0.0), (0.0, 2.0)] {
        let expected = scalar(base_value(base), exponent_value(exponent), num_type, 42).unwrap();
        let actual = vector(
          array(vec![base_value(base)]),
          array(vec![exponent_value(exponent)]),
          num_type,
          42,
        )
        .unwrap();
        assert_eq!(actual.as_array().unwrap()[0], expected);
      }
      assert!(matches!(
        vector(
          array(vec![base_value(0.0), Value::Bool(false)]),
          array(vec![exponent_value(-1.0), exponent_value(1.0)]),
          num_type,
          42
        ),
        Err(VMError::TypeMismatch { ip: 42, .. })
      ));
      for func in [instruction, vector_instruction] {
        let mut stack = Stack::new();
        assert!(matches!(
          func(&mut stack, num_type, 42),
          Err(VMError::StackUnderflow { ip: 42, .. })
        ));
        assert!(stack.is_empty());
      }
    }
  }
}

#[test]
fn floating_power_preserves_ieee_results_and_precision_boundaries() {
  for num_type in FLOATS {
    for (base, exponent) in [
      (f64::NAN, 0.0),
      (1.0, f64::NAN),
      (2.0, f64::NAN),
      (f64::INFINITY, 2.0),
      (f64::NEG_INFINITY, 0.5),
      (-2.0, f64::INFINITY),
      (2.0, 10000.0),
    ] {
      let scalar = arithmetic::powf_func::powf_values(
        Value::Float64(base),
        Value::Float64(exponent),
        num_type,
        42,
      )
      .unwrap()
      .as_f64();
      let vector = vector::arithmetic::powfv_func::powfv_values(
        array(vec![Value::Float64(base)]),
        array(vec![Value::Float64(exponent)]),
        num_type,
        42,
      )
      .unwrap()
      .as_array()
      .unwrap()[0]
        .as_f64();
      assert!(scalar == vector || (scalar.is_nan() && vector.is_nan()));
    }
    for (base, exponent, valid) in [
      (-2.0, 2.0 + 1e-8, !matches!(num_type, PrimitiveTypes::Dbl)),
      (1e-50, -1.0, matches!(num_type, PrimitiveTypes::Dbl)),
      (-1e-50, 0.5, !matches!(num_type, PrimitiveTypes::Dbl)),
    ] {
      for result in [
        arithmetic::powf_func::powf_values(
          Value::Float64(base),
          Value::Float64(exponent),
          num_type,
          42,
        ),
        vector::arithmetic::powfv_func::powfv_values(
          array(vec![Value::Float64(base)]),
          array(vec![Value::Float64(exponent)]),
          num_type,
          42,
        ),
      ] {
        if valid {
          assert!(result.is_ok());
        } else {
          domain_error(result.unwrap_err());
        }
      }
    }
    let exponent = match num_type {
      PrimitiveTypes::Hlf => 65535,
      PrimitiveTypes::Flt => 4294967295,
      _ => 18446744073709551615,
    };
    domain_error(
      arithmetic::powi_func::powi_values(
        Value::Float64(0.0),
        Value::Int128(exponent),
        num_type,
        42,
      )
      .unwrap_err(),
    );
    domain_error(
      vector::arithmetic::powiv_func::powiv_values(
        array(vec![Value::Float64(0.0)]),
        array(vec![Value::Int128(exponent)]),
        num_type,
        42,
      )
      .unwrap_err(),
    );
  }
  let error = arithmetic::powf_func::powf_values(
    Value::Float64(-2.0),
    Value::Float64(0.5),
    PrimitiveTypes::Dbl,
    42,
  )
  .unwrap_err();
  assert!(error.to_string().contains("integral exponent"));
  domain_error(error);
}

#[test]
fn converted_zero_power_bases_and_unsupported_directives_preserve_errors() {
  for num_type in FLOATS {
    for (scalar, instruction, vector, vector_instruction, exponent) in [
      (
        arithmetic::powf_func::powf_values as Binary,
        arithmetic::powf_func::powf_func as Instruction,
        vector::arithmetic::powfv_func::powfv_values as Binary,
        vector::arithmetic::powfv_func::powfv_func as Instruction,
        Value::Float64(-1.0),
      ),
      (
        arithmetic::powi_func::powi_values as Binary,
        arithmetic::powi_func::powi_func as Instruction,
        vector::arithmetic::powiv_func::powiv_values as Binary,
        vector::arithmetic::powiv_func::powiv_func as Instruction,
        Value::Int128(-1),
      ),
    ] {
      for base in [1e-50, -1e-50] {
        if !matches!(num_type, PrimitiveTypes::Dbl) {
          let mut stack = Stack::from_vec(vec![
            Value::Bool(true),
            Value::Float64(base),
            exponent.clone(),
          ]);
          let original = stack.clone();
          domain_error(instruction(&mut stack, num_type, 42).unwrap_err());
          assert_eq!(stack, original);
          let mut stack = Stack::from_vec(vec![
            Value::Bool(true),
            array(vec![Value::Float64(2.0), Value::Float64(base)]),
            array(vec![exponent.clone(), exponent.clone()]),
          ]);
          let original = stack.clone();
          domain_error(vector_instruction(&mut stack, num_type, 42).unwrap_err());
          assert_eq!(stack, original);
        }
      }
      assert!(matches!(
        scalar(
          Value::Float64(0.0),
          exponent.clone(),
          PrimitiveTypes::Str,
          42
        ),
        Err(VMError::TypeMismatch { ip: 42, .. })
      ));
      assert!(matches!(
        vector(
          array(vec![Value::Float64(0.0)]),
          array(vec![exponent]),
          PrimitiveTypes::Str,
          42
        ),
        Err(VMError::TypeMismatch { ip: 42, .. })
      ));
    }
  }
  for num_type in [
    PrimitiveTypes::Sht,
    PrimitiveTypes::Int,
    PrimitiveTypes::Lng,
  ] {
    domain_error(
      arithmetic::pow_func::pow_values(Value::Int128(1i128 << 64), Value::Int128(-1), num_type, 42)
        .unwrap_err(),
    );
    domain_error(
      vector::arithmetic::powv_func::powv_values(
        array(vec![Value::Int128(1i128 << 64)]),
        array(vec![Value::Int128(-1)]),
        num_type,
        42,
      )
      .unwrap_err(),
    );
  }
}
