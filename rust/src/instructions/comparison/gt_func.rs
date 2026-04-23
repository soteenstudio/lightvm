use crate::instructions::comparison::gt::{
  gt_f32in::gt_f32in, gt_f64in::gt_f64in, gt_i32in::gt_i32in, gt_i64in::gt_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};

#[inline]
pub fn gt_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Bool(gt_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(gt_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(gt_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(gt_f32in(a.as_f32(), b.as_f32())),
    _ => Value::Bool(false),
  }
}
