use crate::instructions::math::mul::{
  mul_f32in::mul_f32in, mul_f64in::mul_f64in, mul_i32in::mul_i32in, mul_i64in::mul_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn mul_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Int32(mul_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(mul_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Flt => Value::Float32(mul_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(mul_f64in(a.as_f64(), b.as_f64())),
    _ => Value::Bool(false),
  }
}
