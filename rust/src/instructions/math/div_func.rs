use crate::instructions::math::div::{
  div_f32in::div_f32in, div_f64in::div_f64in, div_i32in::div_i32in, div_i64in::div_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};

#[inline]
pub fn div_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Int32(div_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(div_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Flt => Value::Float32(div_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(div_f64in(a.as_f64(), b.as_f64())),
    _ => Value::Bool(false),
  }
}
