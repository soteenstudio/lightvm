use crate::instructions::math::sub::{
  sub_f32in::sub_f32in, sub_f64in::sub_f64in, sub_i32in::sub_i32in, sub_i64in::sub_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};

#[inline]
pub fn sub_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Int32(sub_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(sub_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Flt => Value::Float32(sub_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(sub_f64in(a.as_f64(), b.as_f64())),
    _ => Value::Bool(false),
  }
}
