use crate::instructions::math::add::{
  add_f32in::add_f32in, add_f64in::add_f64in, add_i32in::add_i32in, add_i64in::add_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};

#[inline]
pub fn add_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  println!("{} {}", a.as_i32(), b.as_i32());
  match num_type {
    PrimitiveTypes::Int => Value::Int32(add_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(add_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Flt => Value::Float32(add_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(add_f64in(a.as_f64(), b.as_f64())),
    _ => Value::Bool(false),
  }
}
